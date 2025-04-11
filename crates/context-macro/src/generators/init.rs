use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{
        accounts::Account,
        constraints::{
            ConstraintAssociatedToken, ConstraintInit, ConstraintMint, ConstraintPayer,
            ConstraintSeeded, ConstraintSeeds, ConstraintSpace, ConstraintToken,
        },
        visitor::ContextVisitor,
    },
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote},
    syn::{parse_quote, punctuated::Punctuated, Expr, Ident, PathSegment, Token},
};

enum InitAccountGeneratorTy {
    TokenAccount {
        is_ata: bool,
        mint: Option<Ident>,
        authority: Option<Expr>,
    },
    Mint {
        decimals: Option<Expr>,
        authority: Option<Expr>,
        freeze_authority: Box<Option<Expr>>,
    },
    Other {
        space: Option<Expr>,
    },
}

struct InitAccountGenerator<'a> {
    name: &'a Ident,
    inner_account_ty: String,
    payer: Option<Ident>,
    is_seeded: bool,
    keys: Option<Punctuated<Expr, Token![,]>>,
    ty: InitAccountGeneratorTy,
}

impl<'a> InitAccountGenerator<'a> {
    pub fn new(account: &'a Account) -> Self {
        let ty = match account.inner_ty.as_str() {
            "Mint" => InitAccountGeneratorTy::Mint {
                authority: None,
                decimals: None,
                freeze_authority: Box::new(None),
            },
            "TokenAccount" => InitAccountGeneratorTy::TokenAccount {
                is_ata: false,
                mint: None,
                authority: None,
            },
            _ => InitAccountGeneratorTy::Other { space: None },
        };

        InitAccountGenerator {
            name: &account.name,
            inner_account_ty: account.inner_ty.clone(),
            payer: None,
            is_seeded: false,
            keys: None,
            ty,
        }
    }

    fn get_seeds(&self) -> Option<TokenStream> {
        let punctuated_keys = self.keys.as_ref()?;

        let seeds = if self.is_seeded {
            let account_ty = format_ident!("{}", self.inner_account_ty);
            quote!(#account_ty::derive_with_bump(#punctuated_keys, &bump))
        } else {
            quote!(seeds!(#punctuated_keys, &bump))
        };

        Some(seeds)
    }

    fn generate_token(&self, account_ty: &PathSegment) -> Result<TokenStream, syn::Error> {
        let name = self.name;

        let Some(ref payer) = self.payer else {
            return Err(syn::Error::new(
                self.name.span(),
                "A payer is needed for the `init` constraint",
            ));
        };
        let maybe_signer = {
            self.get_seeds().map(|seeds| {
                quote! {
                    // TODO: avoid reusing seeds here and in verifications
                    let bump = [bumps.#name];
                    let seeds = #seeds;
                    let signer = instruction::CpiSigner::from(&seeds);
                }
            })
        };

        let signers = if maybe_signer.is_some() {
            quote!(Some(&[signer]))
        } else {
            quote!(None)
        };

        let token = match &self.ty {
            InitAccountGeneratorTy::Mint {
                decimals,
                authority,
                freeze_authority,
            } => {
                let default_decimals = parse_quote!(9);
                let decimals = decimals.as_ref().unwrap_or(&default_decimals);
                let Some(ref authority) = authority else {
                    return Err(syn::Error::new(
                        self.name.span(),
                        "A `authority` need to be specified for the `init` constraint",
                    ));
                };
                let f_auth_token = if let Some(auth) = freeze_authority.as_ref() {
                    quote!(Some(#auth))
                } else {
                    quote!(None)
                };
                quote!(SPLCreate::create_mint(#name, &rent, &#payer, &#authority, #decimals, #f_auth_token, #signers)?)
            }
            InitAccountGeneratorTy::TokenAccount {
                authority,
                mint,
                is_ata,
            } => {
                let Some(ref authority) = authority else {
                    return Err(syn::Error::new(
                        self.name.span(),
                        "A `authority` need to be specified for the `init` constraint",
                    ));
                };
                let Some(ref mint) = mint else {
                    return Err(syn::Error::new(
                        self.name.span(),
                        "A `mint` need to be specified for the `init` constraint",
                    ));
                };

                if *is_ata {
                    quote!(SPLCreate::create_associated_token_account(#name, &#payer, &#mint, &#authority, &system_program, &token_program)?)
                } else {
                    quote!(SPLCreate::create_token_account(#name, &rent, &#payer, &#mint, &#authority, #signers)?)
                }
            }
            InitAccountGeneratorTy::Other { space } => {
                let Some(ref space) = space else {
                    return Err(syn::Error::new(
                        self.name.span(),
                        "A space need to be specified for the `init` constraint",
                    ));
                };

                quote!(SystemCpi::create_account(#name, &rent, &#payer, &crate::ID, #space, #signers)?)
            }
        };

        Ok(quote! {
            let #name: #account_ty = {
                #maybe_signer
                #token
            };
        })
    }
}

impl ContextVisitor for InitAccountGenerator<'_> {
    fn visit_payer(&mut self, contraint: &ConstraintPayer) -> Result<(), syn::Error> {
        self.payer = Some(contraint.target.clone());

        Ok(())
    }

    fn visit_space(&mut self, contraint: &ConstraintSpace) -> Result<(), syn::Error> {
        match &mut self.ty {
            InitAccountGeneratorTy::Other { space } => *space = Some(contraint.space.clone()),
            _ => {
                return Err(syn::Error::new(
                    self.name.span(),
                    "Cannot use `space` constraint with `Mint` or `TokenAccount`.",
                ))
            }
        }

        Ok(())
    }

    fn visit_seeded(&mut self, constraint: &ConstraintSeeded) -> Result<(), syn::Error> {
        self.keys = constraint.0.clone();
        self.is_seeded = true;

        Ok(())
    }

    fn visit_seeds(&mut self, contraint: &ConstraintSeeds) -> Result<(), syn::Error> {
        self.keys = Some(contraint.seeds.clone());

        Ok(())
    }

    fn visit_token(&mut self, constraint: &ConstraintToken) -> Result<(), syn::Error> {
        match &mut self.ty {
            InitAccountGeneratorTy::TokenAccount {
                mint, authority, ..
            } => match constraint {
                ConstraintToken::Mint(ident) => *mint = Some(ident.clone()),
                ConstraintToken::Authority(expr) => *authority = Some(expr.clone()),
            },
            _ => {
                return Err(syn::Error::new(
                    self.name.span(),
                    "Cannot use `token` constraint on non `TokenAccount` account.",
                ))
            }
        }
        Ok(())
    }

    fn visit_associated_token(
        &mut self,
        constraint: &ConstraintAssociatedToken,
    ) -> Result<(), syn::Error> {
        match &mut self.ty {
            InitAccountGeneratorTy::TokenAccount {
                mint,
                authority,
                is_ata,
            } => {
                match constraint {
                    ConstraintAssociatedToken::Mint(ident) => *mint = Some(ident.clone()),
                    ConstraintAssociatedToken::Authority(ident) => {
                        *authority = Some(parse_quote!(#ident))
                    }
                }
                *is_ata = true
            }
            _ => {
                return Err(syn::Error::new(
                    self.name.span(),
                    "Cannot use `associated_token` constraint on non `TokenAccount` account.",
                ))
            }
        }
        Ok(())
    }

    fn visit_mint(&mut self, constraint: &ConstraintMint) -> Result<(), syn::Error> {
        match &mut self.ty {
            InitAccountGeneratorTy::Mint {
                authority,
                decimals,
                freeze_authority,
            } => match constraint {
                ConstraintMint::Authority(expr) => *authority = Some(expr.clone()),
                ConstraintMint::Decimals(expr) => *decimals = Some(expr.clone()),
                ConstraintMint::FreezeAuthority(expr) => {
                    *freeze_authority.as_mut() = Some(expr.clone())
                }
            },
            _ => {
                return Err(syn::Error::new(
                    self.name.span(),
                    "Cannot use `mint` constraint on non `Mint` account.",
                ))
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct InitializationGenerator {
    need_check_system: bool,
    need_check_token: bool,
    need_check_ata: bool,
    has_init: bool,
    result: GeneratorResult,
}

impl InitializationGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    fn check_prerequisite(&self, context: &[Account], program: &str) -> Result<(), syn::Error> {
        let has_system_program = context
            .iter()
            .any(|acc| acc.ty.ident == "Program" && acc.inner_ty == program);

        if !has_system_program {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "Using `init` requires including the `Program<{}>` account",
                    program
                ),
            ));
        }

        Ok(())
    }
}

impl ConstraintGenerator for InitializationGenerator {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        Ok(self.result.clone())
    }
}

impl ContextVisitor for InitializationGenerator {
    fn visit_init(&mut self, _constraint: &ConstraintInit) -> Result<(), syn::Error> {
        self.has_init = true;
        self.need_check_system = true;

        Ok(())
    }

    fn visit_associated_token(
        &mut self,
        _constraint: &ConstraintAssociatedToken,
    ) -> Result<(), syn::Error> {
        self.need_check_ata = true;

        Ok(())
    }

    fn visit_accounts(&mut self, accounts: &Vec<Account>) -> Result<(), syn::Error> {
        for account in accounts {
            self.visit_account(account)?;
        }

        if self.need_check_system {
            self.check_prerequisite(accounts, "System")?;

            if self.need_check_token {
                self.check_prerequisite(accounts, "TokenProgram")?;
            }

            if self.need_check_ata {
                self.check_prerequisite(accounts, "AtaTokenProgram")?;
            }
        }

        Ok(())
    }

    fn visit_account(&mut self, account: &Account) -> Result<(), syn::Error> {
        self.visit_constraints(&account.constraints)?;

        if self.has_init {
            if account.inner_ty == "Mint" || account.inner_ty == "TokenAccount" {
                self.need_check_token = true;
            }

            let mut account_generator = InitAccountGenerator::new(account);
            account_generator.visit_account(account)?;
            self.result
                .at_init
                .extend(account_generator.generate_token(&account.ty)?);
            self.has_init = false;
        }

        Ok(())
    }
}

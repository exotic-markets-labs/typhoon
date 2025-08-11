use {
    crate::{accounts::Account, visitor::ContextVisitor},
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{parse_quote, Expr, Ident},
    typhoon_syn::{
        constraints::{
            ConstraintAssociatedToken, ConstraintMint, ConstraintPayer, ConstraintSeeded,
            ConstraintSeeds, ConstraintSpace, ConstraintToken,
        },
        utils::SeedsExpr,
    },
};

enum InitTokenGeneratorTy {
    TokenAccount {
        is_ata: bool,
        mint: Option<Ident>,
        owner: Option<Expr>,
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

pub struct InitTokenGenerator<'a> {
    account: &'a Account,
    payer: Option<Ident>,
    is_seeded: bool,
    keys: Option<SeedsExpr>,
    ty: InitTokenGeneratorTy,
}

impl<'a> InitTokenGenerator<'a> {
    pub fn new(account: &'a Account) -> Self {
        let ty = match account.inner_ty.as_str() {
            "Mint" => InitTokenGeneratorTy::Mint {
                authority: None,
                decimals: None,
                freeze_authority: Box::new(None),
            },
            "TokenAccount" => InitTokenGeneratorTy::TokenAccount {
                is_ata: false,
                mint: None,
                owner: None,
            },
            _ => InitTokenGeneratorTy::Other { space: None },
        };

        InitTokenGenerator {
            account,
            payer: None,
            is_seeded: false,
            keys: None,
            ty,
        }
    }

    fn get_seeds(&self) -> Option<TokenStream> {
        let punctuated_keys = self.keys.as_ref()?;

        let seeds = if self.is_seeded {
            let account_ty = format_ident!("{}", self.account.inner_ty);
            quote! {
                let seeds = #account_ty::derive_with_bump(#punctuated_keys, &bump);
                let signer = instruction::CpiSigner::from(&seeds);
            }
        } else {
            match punctuated_keys {
                SeedsExpr::Punctuated(punctuated) => {
                    quote! {
                        let seeds = seeds!(#punctuated, &bump);
                        let signer = instruction::CpiSigner::from(&seeds);
                    }
                }
                SeedsExpr::Single(expr) => {
                    quote! {
                        let expr = #expr;
                        let expr_len = expr.len();
                        let mut buffer = [bytes::UNINIT_SEED; MAX_SEEDS];
                        unsafe {
                            for (uninit_byte, &src_byte) in buffer[..expr_len].iter_mut().zip(&expr) {
                                uninit_byte.write(instruction::Seed::from(src_byte));
                            }
                            buffer[expr_len].write(instruction::Seed::from(&bump));
                        }

                        let signer = instruction::CpiSigner::from(unsafe { core::slice::from_raw_parts(buffer.as_ptr() as *const instruction::Seed, expr_len + 1) });
                    }
                }
            }
        };

        Some(seeds)
    }

    pub fn generate(&self) -> Result<TokenStream, syn::Error> {
        let name = &self.account.name;

        let Some(ref payer) = self.payer else {
            return Err(syn::Error::new(
                name.span(),
                "A payer is needed for the `init` constraint",
            ));
        };
        let maybe_signer = {
            self.get_seeds().map(|seeds| {
                let pda_bump = format_ident!("{}_bump", name);
                quote! {
                    // TODO: avoid reusing seeds here and in verifications
                    let bump = [#pda_bump];
                    #seeds
                }
            })
        };

        let signers = if maybe_signer.is_some() {
            quote!(Some(&[signer]))
        } else {
            quote!(None)
        };

        let token = match &self.ty {
            InitTokenGeneratorTy::Mint {
                decimals,
                authority,
                freeze_authority,
            } => {
                let default_decimals = parse_quote!(9);
                let decimals = decimals.as_ref().unwrap_or(&default_decimals);
                let Some(ref authority) = authority else {
                    return Err(syn::Error::new(
                        name.span(),
                        "A `authority` need to be specified for the `init` constraint",
                    ));
                };
                let f_auth_token = if let Some(auth) = freeze_authority.as_ref() {
                    quote!(Some(#auth))
                } else {
                    quote!(None)
                };
                quote!(SplCreateMint::create_mint(#name, &rent, &#payer, &#authority, #decimals, #f_auth_token, #signers)?)
            }
            InitTokenGeneratorTy::TokenAccount {
                owner,
                mint,
                is_ata,
            } => {
                let Some(ref owner) = owner else {
                    return Err(syn::Error::new(
                        name.span(),
                        "A `authority` need to be specified for the `init` constraint",
                    ));
                };
                let Some(ref mint) = mint else {
                    return Err(syn::Error::new(
                        name.span(),
                        "A `mint` need to be specified for the `init` constraint",
                    ));
                };

                if *is_ata {
                    quote!(SplCreateToken::create_associated_token_account(#name, &#payer, &#mint, &#owner, &system_program, &token_program)?)
                } else {
                    quote!(SplCreateToken::create_token_account(#name, &rent, &#payer, &#mint, &#owner, #signers)?)
                }
            }
            InitTokenGeneratorTy::Other { space } => {
                let account_ty = format_ident!("{}", self.account.inner_ty);
                let default_space = parse_quote!(#account_ty::SPACE);
                let space = space.as_ref().unwrap_or(&default_space);

                quote!(CreateAccountCpi::create(#name, &rent, &#payer, &program_id, #space, #signers)?)
            }
        };

        Ok(quote! {
            #maybe_signer
            #token
        })
    }
}

impl ContextVisitor for InitTokenGenerator<'_> {
    fn visit_payer(&mut self, contraint: &ConstraintPayer) -> Result<(), syn::Error> {
        self.payer = Some(contraint.target.clone());

        Ok(())
    }

    fn visit_space(&mut self, contraint: &ConstraintSpace) -> Result<(), syn::Error> {
        match &mut self.ty {
            InitTokenGeneratorTy::Other { space } => *space = Some(contraint.space.clone()),
            _ => {
                return Err(syn::Error::new(
                    self.account.name.span(),
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
            InitTokenGeneratorTy::TokenAccount { mint, owner, .. } => match constraint {
                ConstraintToken::Mint(ident) => *mint = Some(ident.clone()),
                ConstraintToken::Owner(expr) => *owner = Some(expr.clone()),
            },
            _ => {
                return Err(syn::Error::new(
                    self.account.name.span(),
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
            InitTokenGeneratorTy::TokenAccount {
                mint,
                owner,
                is_ata,
            } => {
                match constraint {
                    ConstraintAssociatedToken::Mint(ident) => *mint = Some(ident.clone()),
                    ConstraintAssociatedToken::Authority(ident) => {
                        *owner = Some(parse_quote!(#ident))
                    }
                }
                *is_ata = true
            }
            _ => {
                return Err(syn::Error::new(
                    self.account.name.span(),
                    "Cannot use `associated_token` constraint on non `TokenAccount` account.",
                ))
            }
        }
        Ok(())
    }

    fn visit_mint(&mut self, constraint: &ConstraintMint) -> Result<(), syn::Error> {
        match &mut self.ty {
            InitTokenGeneratorTy::Mint {
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
                    self.account.name.span(),
                    "Cannot use `mint` constraint on non `Mint` account.",
                ))
            }
        }
        Ok(())
    }
}

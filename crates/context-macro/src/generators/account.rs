use {
    crate::accounts::Account,
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{parse_quote, Expr, Ident},
    typhoon_syn::{
        account_meta::AccountMeta,
        error,
        utils::{ContextExpr, SeedsExpr},
    },
};

pub enum AccountType {
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
        targets: Vec<(Ident, Option<Expr>)>,
    },
}

#[derive(Default)]
pub struct InitContext {
    pub is_init_if_needed: bool,
    pub payer: Option<Ident>,
}

#[derive(Default)]
pub struct PdaContext {
    pub keys: Option<SeedsExpr>,
    pub bump: Option<ContextExpr>,
    pub is_seeded: bool,
    pub program_id: Option<Expr>,
}

pub struct AccountGenerator<'a> {
    pub account: &'a Account,
    pub account_ty: AccountType,
    pub init: Option<InitContext>,
    pub pda: Option<PdaContext>,
    pub meta: AccountMeta,
    pub init_state: bool,
}

impl<'a> AccountGenerator<'a> {
    pub fn new(account: &'a Account, meta: AccountMeta, account_ty: AccountType) -> Self {
        Self {
            account,
            account_ty,
            init: None,
            pda: Default::default(),
            meta,
            init_state: false,
        }
    }
}

impl AccountGenerator<'_> {
    pub fn needs_programs(&self) -> Vec<String> {
        let mut programs = Vec::with_capacity(3);
        if self.init.is_some() {
            programs.push("System".to_string());
            match self.account_ty {
                AccountType::TokenAccount { is_ata, .. } => {
                    programs.push("TokenProgram".to_string());

                    if is_ata {
                        programs.push("AtaTokenProgram".to_string());
                    }
                }
                AccountType::Mint { .. } => programs.push("TokenProgram".to_string()),
                _ => (),
            }
        }
        programs
    }

    fn get_pda(&self, ctx: &PdaContext, find: bool) -> Result<TokenStream, syn::Error> {
        let name = &self.account.name;
        let pda_bump = format_ident!("{}_bump", name);
        let program_id = ctx
            .program_id
            .as_ref()
            .map(|p| quote!(#p))
            .unwrap_or(quote!(program_id));

        match ctx.bump {
            Some(ref bump) if !find => {
                let pda_key = format_ident!("{}_key", name);
                let seeds_token = if ctx.is_seeded {
                    let var_name = format_ident!("{}_state", self.account.name);
                    quote!(#var_name.seeds_with_bump(&[#pda_bump]))
                } else {
                    let Some(ref seed_keys) = ctx.keys else {
                        error!(
                            &self.account.name,
                            "No seeds specified for the current PDA."
                        );
                    };

                    match seed_keys {
                        SeedsExpr::Punctuated(punctuated) => quote!([#punctuated, &[#pda_bump]]),
                        SeedsExpr::Single(expr) => quote!(#expr(&[#pda_bump])),
                    }
                };

                Ok(quote! {
                    let #pda_bump = #bump;
                    let #pda_key = create_program_address(&#seeds_token, &#program_id)?;
                })
            }
            _ => {
                let Some(ref seed_keys) = ctx.keys else {
                    error!(
                        &self.account.name,
                        "No seeds specified for the current PDA."
                    );
                };

                let seeds_token = if ctx.is_seeded {
                    let inner_ty = format_ident!("{}", self.account.inner_ty);
                    quote!(#inner_ty::derive(#seed_keys))
                } else {
                    match seed_keys {
                        SeedsExpr::Punctuated(punctuated) => quote!([#punctuated]),
                        SeedsExpr::Single(expr) => quote!(#expr),
                    }
                };
                Ok(quote! {
                    let (_, #pda_bump) = find_program_address(&#seeds_token, &#program_id);
                })
            }
        }
    }

    fn get_signer_init(&self, ctx: &PdaContext) -> Result<TokenStream, syn::Error> {
        let pda_bump = format_ident!("{}_bump", self.account.name);
        let punctuated_keys = ctx.keys.as_ref().ok_or(syn::Error::new_spanned(
            &self.account.name,
            "The seeds cannot be empty.",
        ))?;

        let seeds = if ctx.is_seeded {
            let account_ty = format_ident!("{}", self.account.inner_ty);
            quote! {
                let seeds = #account_ty::derive_signer_seeds_with_bump(#punctuated_keys, &bump);
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
                        for (uninit_byte, &src_byte) in buffer[..expr_len].iter_mut().zip(&expr) {
                            uninit_byte.write(instruction::Seed::from(src_byte));
                        }
                        buffer[expr_len].write(instruction::Seed::from(&bump));

                        let signer = instruction::CpiSigner::from(unsafe { core::slice::from_raw_parts(buffer.as_ptr() as *const instruction::Seed, expr_len + 1) });
                    }
                }
            }
        };

        Ok(quote! {
            // TODO: avoid reusing seeds here and in verifications
            let bump = [#pda_bump];
            #seeds
        })
    }

    fn get_init_token(
        &self,
        ctx: &InitContext,
        signers: TokenStream,
    ) -> Result<TokenStream, syn::Error> {
        let name = &self.account.name;
        if !self.meta.is_mutable {
            //TODO add signer when it's ready
            error!(name, "The account needs to be mutable");
        }
        let payer = ctx.payer.as_ref().ok_or(syn::Error::new_spanned(
            name,
            "A payer needs to be specified for `init` or init_if_needed` constraint.",
        ))?;

        let init_token = match &self.account_ty {
            AccountType::TokenAccount {
                is_ata,
                mint,
                owner,
            } => {
                let Some(owner) = owner else {
                    error!(name, "An `owner` need to be specified for the `init` or `init_if_needed` constraint.");
                };
                let Some(mint) = mint else {
                    error!(name, "A `mint` need to be specified for the `init` or `init_if_needed` constraint.");
                };

                if *is_ata {
                    quote!(SplCreateToken::create_associated_token_account(#name, &#payer, &#mint, &#owner, &system_program, &token_program)?)
                } else {
                    quote!(SplCreateToken::create_token_account(#name, &rent, &#payer, &#mint, &#owner, #signers)?)
                }
            }
            AccountType::Mint {
                decimals,
                authority,
                freeze_authority,
            } => {
                let default_decimals = parse_quote!(9);
                let decimals = decimals.as_ref().unwrap_or(&default_decimals);
                let Some(authority) = authority else {
                    error!(name, "An `authority` need to be specified for the `init` or `init_if_needed` constraint.");
                };
                let f_auth_token = if let Some(auth) = freeze_authority.as_ref() {
                    quote!(Some(#auth))
                } else {
                    quote!(None)
                };
                quote!(SplCreateMint::create_mint(#name, &rent, &#payer, &#authority, #decimals, #f_auth_token, #signers)?)
            }
            AccountType::Other { space, .. } => {
                let account_ty = format_ident!("{}", self.account.inner_ty);
                let default_space = parse_quote!(#account_ty::SPACE);
                let space = space.as_ref().unwrap_or(&default_space);
                quote!(CreateAccountCpi::create(#name, &rent, &#payer, &program_id, #space, #signers)?)
            }
        };

        Ok(init_token)
    }

    pub fn account_token(&self) -> Result<TokenStream, syn::Error> {
        let mut token = TokenStream::new();
        let name = &self.account.name;
        let name_str = name.to_string();
        let account_ty = &self.account.ty;
        let var_name = format_ident!("{}_state", name);
        let pda_key = format_ident!("{}_key", name);

        token.extend(quote!(let #name = <#account_ty as FromAccountInfo>::try_from_info(#name).trace_account(#name_str)?;));

        if self.init_state {
            token.extend(quote!(let #var_name = #name.data_unchecked()?;));
        }

        if let Some(ref pda_ctx) = self.pda {
            let pda = self.get_pda(pda_ctx, false)?;
            token.extend(pda);
            token.extend(quote! {
                if #name.key() != &#pda_key {
                    return Err(Error::new(ProgramError::InvalidSeeds).with_account(#name_str));
                }
            });
        }

        match self.account_ty {
            AccountType::TokenAccount {
                ref mint,
                ref owner,
                ..
            } => {
                if let Some(mint) = mint {
                    token.extend(quote! {
                        if #var_name.mint() != #mint.key() {
                            return Err(ErrorCode::TokenConstraintViolated.into());
                        }
                    });
                }

                token.extend(quote! {
                    if #var_name.owner() != #owner.key() {
                        return Err(ErrorCode::TokenConstraintViolated.into());
                    }
                });
            }
            AccountType::Mint { .. } => {}
            AccountType::Other { ref targets, .. } => {
                let basic_error: Expr = parse_quote!(ErrorCode::HasOneConstraint);
                let targets = targets.iter().map(|(target, error)| {
                    let target = &target;
                    let error = error.as_ref().unwrap_or(&basic_error);

                    quote! {
                        if &#var_name.#target != #target.key() {
                            return Err(#error.into());
                        }
                    }
                });
                token.extend(targets);
            }
        }
        Ok(token)
    }

    fn needs_return_bump(&self) -> bool {
        let has_pda = self.pda.is_some();
        let pda_has_no_bump = self
            .pda
            .as_ref()
            .map(|el| el.bump.is_none())
            .unwrap_or_default();
        let is_init_if_needed = self
            .init
            .as_ref()
            .map(|el| el.is_init_if_needed)
            .unwrap_or_default();

        has_pda && (pda_has_no_bump || is_init_if_needed)
    }

    pub fn generate(self) -> Result<TokenStream, syn::Error> {
        let mut token = TokenStream::new();
        let name = &self.account.name;
        let account_ty = &self.account.ty;
        let pda_bump = format_ident!("{}_bump", name);

        let return_ty = if self.needs_return_bump() {
            quote!((#name, #pda_bump))
        } else {
            quote!(#name)
        };

        let account_checks_token = if let Some(ref init_ctx) = self.init {
            let signers = if self.pda.is_some() {
                quote!(Some(&[signer]))
            } else {
                quote!(None)
            };
            let init_token = self.get_init_token(init_ctx, signers)?;
            let init_account_token = if let Some(ref pda_ctx) = self.pda {
                let pda_token = self.get_pda(pda_ctx, init_ctx.is_init_if_needed)?;
                let seeds_token = self.get_signer_init(pda_ctx)?;
                quote! {
                    #pda_token
                    #seeds_token
                    let #name = { #init_token };
                }
            } else {
                quote! {
                    let #name: #account_ty = {
                        #init_token
                    };
                }
            };

            if init_ctx.is_init_if_needed {
                let account_token = self.account_token()?;
                quote! {
                    let #return_ty = if !#name.is_owned_by(&Pubkey::default()) {
                        #account_token
                        #return_ty
                    }else {
                        #init_account_token
                        #return_ty
                    };
                }
            } else {
                quote!(#init_account_token)
            }
        } else {
            self.account_token()?
        };

        if self.meta.is_optional {
            token.extend(quote! {
                let #return_ty = if #name.key() == program_id {
                    None
                } else {
                    #account_checks_token
                    Some(#return_ty)
                };
            });
        } else {
            token.extend(account_checks_token);
        };

        Ok(token)
    }
}

/*
ConstraintAssociatedToken::Mint(ident) => {
    if let Some(seeds) = self.seeds.as_mut() {
        let SeedsExpr::Punctuated(expr) = seeds else {
            return Err(syn::Error::new_spanned(
                &seeds,
                "Seeds expr cannot be used in this context.",
            ));
        };

        expr.insert(2, parse_quote!(#ident));
    } else {
        self.seeds = Some(SeedsExpr::Punctuated(
            parse_quote!(token_program.key().as_ref(), #ident),
        ));
        self.program_id = Some(parse_quote!(AtaTokenProgram::ID))
    }
}
ConstraintAssociatedToken::Authority(ident) => {
    if let Some(seeds) = self.seeds.as_mut() {
        let SeedsExpr::Punctuated(expr) = seeds else {
            return Err(syn::Error::new_spanned(
                &seeds,
                "Seeds expr cannot be used in this context.",
            ));
        };

        expr.insert(0, parse_quote!(#ident));
    } else {
        self.seeds = Some(SeedsExpr::Punctuated(
            parse_quote!(#ident, token_program.key().as_ref()),
        ));
        self.program_id = Some(parse_quote!(AtaTokenProgram::ID))
    }
}

*/

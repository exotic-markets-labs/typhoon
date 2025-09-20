use {
    crate::{
        generators::account::{AccountGenerator, AccountType, InitContext, PdaContext},
        ParsingContext,
    },
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    std::collections::HashSet,
    syn::{parse_quote, Ident, ItemStruct},
    typhoon_syn::{
        constraints::{Constraint, ConstraintAssociatedToken, ConstraintMint, ConstraintToken},
        error, Argument, Arguments,
    },
};

pub struct GlobalContext<'a> {
    pub need_rent: bool,
    pub accounts: Vec<AccountGenerator<'a>>,
    pub bumps: HashSet<String>,
    pub program_checks: HashSet<String>,
}

impl<'a> GlobalContext<'a> {
    pub fn generate_args(&self, context: &ParsingContext) -> Option<(Ident, Option<TokenStream>)> {
        let args = context.args.as_ref()?;

        let result = match args {
            Arguments::Struct(name) => (name.clone(), None),
            Arguments::Values(args) => {
                let struct_name = format_ident!("{}Args", context.item_struct.ident);
                let fields = args
                    .iter()
                    .map(|Argument { name, ty }: &Argument| quote!(pub #name: #ty));

                let generated_struct = quote! {
                    #[derive(Debug, PartialEq, bytemuck::AnyBitPattern, bytemuck::NoUninit, Copy, Clone)]
                    #[repr(C)]
                    pub struct #struct_name {
                        #(#fields),*
                    }
                };

                (struct_name, Some(generated_struct))
            }
        };

        Some(result)
    }

    pub fn generate_bumps(&self, context: &ParsingContext) -> Option<(ItemStruct, TokenStream)> {
        if self.bumps.is_empty() {
            return None;
        }

        let struct_name = format_ident!("{}Bumps", context.item_struct.ident);
        let struct_fields = self.bumps.iter().map(|el| format_ident!("{}", el));
        let bumps_struct = parse_quote! {
            #[derive(Debug, PartialEq)]
            pub struct #struct_name {
                #(pub #struct_fields: u8,)*
            }
        };

        let assign_fields = self.bumps.iter().map(|n| {
            let name = format_ident!("{n}");
            let bump_ident = format_ident!("{n}_bump");
            quote!(#name: #bump_ident)
        });
        let bumps_var = quote! {
            let bumps = #struct_name {
                #(#assign_fields),*
            };
        };

        Some((bumps_struct, bumps_var))
    }

    pub fn from_parsing_context(context: &'a ParsingContext) -> Result<Self, syn::Error> {
        let mut need_rent = false;
        let mut accounts: Vec<AccountGenerator<'_>> = Vec::new();
        let mut bumps = HashSet::new();
        let mut program_checks = HashSet::new();
        let mut states = HashSet::new();

        //TODO optimize sorting etc..
        for account in &context.accounts {
            let account_ty = match account.inner_ty.to_string().as_str() {
                "TokenAccount" => AccountType::TokenAccount {
                    is_ata: false,
                    mint: None,
                    owner: None,
                },
                "Mint" => AccountType::Mint {
                    authority: None,
                    decimals: None,
                    freeze_authority: Box::new(None),
                },
                _ => AccountType::Other {
                    space: None,
                    targets: vec![],
                },
            };
            let mut generator = AccountGenerator::new(account, account_ty);
            let name = &account.name;

            for constraint in &account.constraints.0 {
                match constraint {
                    Constraint::Init(_) => {
                        if generator.init.is_some() {
                            error!(
                                name,
                                "The `init` or `init_if_needed` constraint is already specified."
                            );
                        }
                        generator.init = Some(InitContext::default());
                    }
                    Constraint::Payer(constraint_payer) => {
                        if let Some(init_ctx) = &mut generator.init {
                            if matches!(generator.account_ty, AccountType::Other { .. }) {
                                need_rent = true;
                            }
                            init_ctx.payer = Some(constraint_payer.target.to_owned());
                        } else {
                            error!(
                                name,
                                "`payer` can only be used with `init` or `init_if_needed` constraint."
                            )
                        }
                    }
                    Constraint::Space(constraint_space) => {
                        if generator.init.is_none() {
                            error!(name, "`space` can only be specified with `init` or `init_if_needed` constraint.");
                        }

                        if let AccountType::Other { space, .. } = &mut generator.account_ty {
                            *space = Some(constraint_space.space.to_owned())
                        } else {
                            error!(
                                name,
                                "`space` cannot be used on `Mint` or `TokenAccount` type."
                            )
                        }
                    }
                    Constraint::Seeded(constraint_seeded) => {
                        if generator.pda.is_some() {
                            error!(name, "`seeds` or `seeded` are already defined.")
                        }

                        generator.pda = Some(PdaContext {
                            keys: constraint_seeded.0.to_owned(),
                            bump: None,
                            is_seeded: true,
                            program_id: None,
                        });
                    }
                    Constraint::Seeds(constraint_seeds) => {
                        if generator.pda.is_some() {
                            error!(name, "`seeds` or `seeded` are already defined.")
                        }

                        generator.pda = Some(PdaContext {
                            keys: Some(constraint_seeds.seeds.to_owned()),
                            bump: None,
                            is_seeded: false,
                            program_id: None,
                        });
                    }
                    Constraint::Bump(constraint_bump) => {
                        if let Some(pda_ctx) = &mut generator.pda {
                            match &constraint_bump.0 {
                                Some(expr) => {
                                    if generator
                                        .init
                                        .as_ref()
                                        .map(|el| el.is_init_if_needed)
                                        .unwrap_or_default()
                                    {
                                        bumps.insert(name.to_string());
                                    }
                                    pda_ctx.bump = Some(expr.to_owned());
                                    if let Some(name) = expr.name() {
                                        states.insert(name.to_string());
                                    }
                                }
                                None => {
                                    bumps.insert(name.to_string());
                                }
                            }
                        } else {
                            error!(name, "`bump` can only be used in a PDA context.");
                        }
                    }
                    Constraint::HasOne(constraint_has_one) => {
                        if let AccountType::Other { targets, .. } = &mut generator.account_ty {
                            targets.push((
                                constraint_has_one.join_target.to_owned(),
                                constraint_has_one.error.clone(),
                            ));
                            states.insert(name.to_string());
                        } else {
                            error!(
                                name,
                                "`has_one` cannot be used on `Mint` or `TokenAccount` type."
                            );
                        }
                    }
                    Constraint::Program(constraint_program) => {
                        if let Some(pda_ctx) = &mut generator.pda {
                            pda_ctx.program_id = Some(constraint_program.0.to_owned());
                        } else {
                            error!(name, "`program` can only be used in a PDA context.");
                        }
                    }
                    Constraint::Token(constraint_token) => {
                        if let AccountType::TokenAccount {
                            is_ata,
                            mint,
                            owner,
                        } = &mut generator.account_ty
                        {
                            if *is_ata {
                                error!(name, "`associated_token` is already defined.");
                            }

                            if generator.init.is_none() {
                                states.insert(name.to_string());
                            }

                            match constraint_token {
                                ConstraintToken::Mint(ident) => {
                                    *mint = Some(ident.to_owned());
                                }
                                ConstraintToken::Owner(expr) => *owner = Some(expr.to_owned()),
                            }
                        } else {
                            error!(
                                name,
                                "`token` can only be used with the `TokenAccount` type."
                            );
                        }
                    }
                    Constraint::Mint(constraint_mint) => {
                        if let AccountType::Mint {
                            decimals,
                            authority,
                            freeze_authority,
                        } = &mut generator.account_ty
                        {
                            states.insert(name.to_string());

                            match constraint_mint {
                                ConstraintMint::Authority(expr) => {
                                    *authority = Some(expr.to_owned())
                                }
                                ConstraintMint::Decimals(expr) => *decimals = Some(expr.to_owned()),
                                ConstraintMint::FreezeAuthority(expr) => {
                                    *freeze_authority = Box::new(Some(expr.to_owned()))
                                }
                            }
                        } else {
                            error!(
                                name,
                                "`mint` constraint can only be used with the `Mint` type"
                            )
                        }
                    }
                    Constraint::AssociatedToken(constraint_associated_token) => {
                        if let AccountType::TokenAccount {
                            mint,
                            owner,
                            is_ata,
                        } = &mut generator.account_ty
                        {
                            *is_ata = true;
                            states.insert(name.to_string());

                            match constraint_associated_token {
                                ConstraintAssociatedToken::Mint(ident) => {
                                    *mint = Some(ident.to_owned());
                                }
                                ConstraintAssociatedToken::Authority(ident) => {
                                    *owner = Some(parse_quote!(#ident));
                                }
                            }
                        } else {
                            error!(
                                name,
                                "`associated_token` can only be used with the `TokenAccount` type."
                            );
                        }
                    }
                    Constraint::InitIfNeeded(_) => {
                        if generator.init.is_some() {
                            error!(
                                name,
                                "The `init` or `init_if_needed` constraint is already specified."
                            );
                        }

                        generator.init = Some(InitContext {
                            is_init_if_needed: true,
                            payer: None,
                        })
                    }
                }
            }

            for program in generator.needs_programs() {
                program_checks.insert(program);
            }
            accounts.push(generator);
        }

        for state in states.iter() {
            for account in &mut accounts {
                if &account.account.name.to_string() == state {
                    account.init_state = true
                }
            }
        }

        Ok(GlobalContext {
            need_rent,
            accounts,
            bumps,
            program_checks,
        })
    }
}

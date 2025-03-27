use {
    crate::{accounts::Account, extractor::AccountExtractor},
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::Ident,
};

pub struct Bumps(pub Vec<Account>);

impl Bumps {
    pub fn get_name(&self, context_name: &Ident) -> Ident {
        format_ident!("{}Bumps", context_name)
    }

    pub fn generate_struct(&self, context_name: &Ident) -> TokenStream {
        let struct_name = self.get_name(context_name);
        let mut fields = Vec::with_capacity(self.0.len());

        for a in &self.0 {
            if !a.constraints.is_seeded() || a.constraints.has_init() {
                let name = &a.name;
                fields.push(quote! {
                    pub #name: u8,
                });
            }
        }

        quote! {
            #[derive(Debug, PartialEq)]
            pub struct #struct_name {
                #(#fields)*
            }
        }
    }

    pub fn get_checks(&self) -> TokenStream {
        let mut checks = Vec::with_capacity(self.0.len());

        for a in &self.0 {
            let c = &a.constraints;
            let name = &a.name;
            let pk_ident = format_ident!("{}_pk", name);

            let check = match (c.must_find_canonical_bump(), c.is_seeded(), c.has_init()) {
                (true, _, _) | (_, true, true) => Some(quote! {
                    if #name.key() != &#pk_ident {
                        return Err(ProgramError::InvalidSeeds);
                    }
                }),
                (_, true, false) => Some(quote! {
                    let (#pk_ident, _) = find_program_address(&#name.data()?.seeds(), &crate::ID);
                    if #name.key() != &#pk_ident {
                        return Err(ProgramError::InvalidSeeds);
                    }
                }),
                _ => {
                    let seeds = c.get_seeds();
                    let bump = c.get_bump(name);

                    if let (Some(seeds), Some(bump)) = (seeds, bump) {
                        Some(quote! {
                            let #pk_ident = create_program_address(&[#seeds, &[#bump]], &crate::ID)?;
                            if #name.key() != &#pk_ident {
                                return Err(ProgramError::InvalidSeeds);
                            }
                        })
                    } else {
                        None
                    }
                }
            };

            if let Some(check) = check {
                checks.push(check);
            }
        }

        quote! {
            #(#checks)*
        }
    }

    pub fn get_assign(&self, context_name: &Ident) -> TokenStream {
        let mut creations = Vec::with_capacity(self.0.len());
        let mut values = Vec::with_capacity(self.0.len());

        for a in &self.0 {
            let c = &a.constraints;
            let name = &a.name;
            let pk_ident = format_ident!("{}_pk", name);
            let bump_ident = format_ident!("{}_bump", name);

            if c.is_seeded() {
                if let Some(keys) = c.get_keys() {
                    let account_ty = AccountExtractor(&a.ty).get_account_type();

                    creations.push(quote! {
                        let (#pk_ident, #bump_ident) = find_program_address(&#account_ty::derive(#keys), &crate::ID);
                    });
                    values.push(quote! {
                        #name: #bump_ident,
                    });
                }
                continue;
            }

            if c.must_find_canonical_bump() {
                if let Some(seeds) = c.get_seeds() {
                    creations.push(quote! {
                        let (#pk_ident, #bump_ident) = find_program_address(&[#seeds], &crate::ID);
                    });
                    values.push(quote! {
                        #name: #bump_ident,
                    });
                } else {
                    creations.push(
                        syn::Error::new(
                            a.name.span(),
                            "Seeds must be provided to generate bump assignments",
                        )
                        .to_compile_error(),
                    );
                }
                continue;
            }

            if let Some(bump) = c.get_bump(name) {
                values.push(quote! {
                    #name: #bump,
                });
            } else {
                values.push(
                    syn::Error::new(
                        a.name.span(),
                        "A bump must be provided to generate key checks",
                    )
                    .to_compile_error(),
                );
            }
        }

        let struct_name = self.get_name(context_name);

        quote! {
            #(#creations)*

            let bumps = #struct_name {
                #(#values)*
            };
        }
    }
}

impl TryFrom<&Vec<Account>> for Bumps {
    type Error = syn::Error;

    fn try_from(accounts: &Vec<Account>) -> Result<Self, Self::Error> {
        let mut filtered = Vec::with_capacity(accounts.len());
        for account in accounts {
            let c = &account.constraints;
            if (c.has_init() && c.get_seeds().is_some()) || c.is_seeded() {
                filtered.push(account.clone());
            }
        }
        Ok(Bumps(filtered))
    }
}

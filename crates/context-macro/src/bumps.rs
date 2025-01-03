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
        let fields = self.0.iter().filter_map(|a| {
            if a.constraints.is_seeded() && !a.constraints.has_init() {
                None
            } else {
                let name = &a.name;
                Some(quote! {
                    pub #name: u8,
                })
            }
        });
        let struct_name = self.get_name(context_name);

        quote! {
            #[derive(Debug, PartialEq)]
            pub struct #struct_name {
                #(#fields)*
            }
        }
    }

    pub fn get_checks(&self) -> TokenStream {
        let checks = self
            .0
            .iter()
            .filter_map(|a| {
                let c = &a.constraints;
                let name = &a.name;
                let pk_ident = format_ident!("{}_pk", name);

                match (c.must_find_canonical_bump(), c.is_seeded(), c.has_init()) {
                    (true, _, _) | (_, true, true) => Some(quote! {
                        if #name.key() != &#pk_ident {
                            return Err(ProgramError::InvalidSeeds);
                        }
                    }),
                    (_, true, false) => Some(quote! {
                        let (#pk_ident, _) = typhoon_program::pubkey::find_program_address(&#name.data()?.seeds(), &crate::ID);
                        if #name.key() != &#pk_ident {
                            return Err(ProgramError::InvalidSeeds);
                        }
                    }),
                    _ => {
                        if let (Some(seeds), Some(bump)) = (c.get_seeds(), c.get_bump(name)) {
                            Some(quote! {
                                let #pk_ident = typhoon_program::pubkey::create_program_address(&[#seeds, &[#bump]], &crate::ID)?;
                                if #name.key() != &#pk_ident {
                                    return Err(ProgramError::InvalidSeeds);
                                }
                            })
                        } else {
                            None
                        }
                    }
                }
            })
            .collect::<Vec<TokenStream>>();

        quote! {
            #(#checks)*
        }
    }

    pub fn get_assign(&self, context_name: &Ident) -> TokenStream {
        let (creations, values): (Vec<TokenStream>, Vec<TokenStream>) = self.0.iter().map(|a| {
            let c = &a.constraints;
            let name = &a.name;
            let pk_ident = format_ident!("{}_pk", name);
            let bump_ident = format_ident!("{}_bump", name);

            // TODO: do not always compute the bump when account is seeded
            if c.is_seeded() {
                if let Some(keys) = c.get_keys() {
                    let account_ty = AccountExtractor(&a.ty).get_account_type();

                    (quote! {
                        let (#pk_ident, #bump_ident) = typhoon_program::pubkey::find_program_address(&#account_ty::derive(#keys), &crate::ID);
                    }, quote! {
                        #name: #bump_ident,
                    })
                } else {
                    (quote!(), quote!())
                }
            } else if c.must_find_canonical_bump() {
                (if let Some(seeds) = c.get_seeds() {
                    quote! {
                        let (#pk_ident, #bump_ident) = typhoon_program::pubkey::find_program_address(&[#seeds], &crate::ID);
                    }
                } else {
                    syn::Error::new(a.name.span(), "Seeds must be provided to generate bump assignments").to_compile_error()
                }, quote! {
                    #name: #bump_ident,
                })
            } else {
                (quote!(),
                if let Some(bump) = a.constraints.get_bump(name) {
                    quote! {
                        #name: #bump,
                    }
                } else {
                    syn::Error::new(a.name.span(), "A bump must be provided to generate key checks").to_compile_error()
                })
            }
        }).unzip();

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

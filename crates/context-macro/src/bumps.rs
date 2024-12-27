use {
    crate::accounts::Account,
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{Expr, Ident},
};

pub struct Bumps(pub Vec<(Ident, Expr, TokenStream)>);

impl Bumps {
    pub fn get_name(&self, context_name: &Ident) -> Ident {
        format_ident!("{}Bumps", context_name)
    }

    pub fn generate_struct(&self, context_name: &Ident) -> TokenStream {
        let fields = self.0.iter().map(|(name, _, _)| {
            quote! {
                pub #name: u64, // TODO: fix alignment issues
            }
        });
        let struct_name = self.get_name(context_name);

        quote! {
            #[repr(C)]
            #[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
            pub struct #struct_name {
                #(#fields)*
            }
        }
    }

    pub fn get_checks_and_assigns(&self, context_name: &Ident) -> (TokenStream, TokenStream) {
        let ((creation, checks), values): ((Vec<TokenStream>, Vec<TokenStream>), Vec<TokenStream>) = self.0.iter().map(|(name, value, seeds)| {
            let pk = format_ident!("{}_pk", name);
            let bump = format_ident!("{}_bump", name);

            let computed_bump = (
                (quote! {
                    let (#pk, #bump) = typhoon_program::pubkey::find_program_address(&[#seeds], &crate::ID);
                },
                quote ! {
                    if #name.key() != &#pk {
                        return Err(ProgramError::InvalidSeeds);
                    }
                }), quote! {
                    #name: #bump as u64, // TODO: Fix alignment
                },
            );
            let provided_bump = (
                (quote! {},
                quote! {
                    let #pk = typhoon_program::pubkey::create_program_address(&[#seeds, &[#value]], &crate::ID)?;
                    if #name.key() != &#pk {
                        return Err(ProgramError::InvalidSeeds);
                    }
                }),
                quote! {
                    #name: #value,
                },
            );

            match value {
                Expr::Path(e) => {
                    if let Some(ident) = e.path.get_ident() {
                        if  bump == *ident {
                            computed_bump
                        } else {
                            provided_bump
                        }
                    } else {
                        provided_bump
                    }
                },
                _ => {
                    provided_bump
                }
            }
        }).unzip();

        let checks = quote! {
            #(#checks)*
        };

        let struct_name = self.get_name(context_name);
        let assign = quote! {
            #(#creation)*

            let bumps = #struct_name {
                #(#values)*
            };
        };

        (checks, assign)
    }
}

impl TryFrom<&Vec<Account>> for Bumps {
    type Error = syn::Error;

    fn try_from(accounts: &Vec<Account>) -> Result<Self, Self::Error> {
        Ok(Bumps(
            accounts
                .iter()
                .filter_map(|a| {
                    if !a.constraints.has_init() {
                        None
                    } else if a.constraints.get_bump(&a.name).is_some()
                        && a.constraints.get_seeds().is_some()
                    {
                        let seeds = a.constraints.get_seeds().unwrap().clone();
                        Some((
                            a.name.clone(),
                            a.constraints.get_bump(&a.name).unwrap().clone(),
                            quote! { #seeds },
                        ))
                    } else if a.constraints.is_seeded() && a.constraints.get_keys().is_some() {
                        let ident = format_ident!("{}_bump", a.name);
                        let keys = a
                            .constraints
                            .get_keys()
                            .unwrap()
                            .iter()
                            .map(|k| quote! { #k.as_ref() })
                            .collect::<syn::punctuated::Punctuated<_, syn::Token![,]>>();
                        let name = a.name.to_string();
                        Some((
                            a.name.clone(),
                            Expr::Path(syn::ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments: vec![syn::PathSegment {
                                        ident,
                                        arguments: syn::PathArguments::None,
                                    }]
                                    .into_iter()
                                    .collect(),
                                },
                            }),
                            quote! { #name.as_bytes(), #keys },
                        ))
                    } else {
                        None
                    }
                })
                .collect(),
        ))
    }
}

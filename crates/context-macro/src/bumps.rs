use {
    crate::accounts::Account,
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{punctuated::Punctuated, Expr, Ident, Token},
};

pub struct Bumps(Vec<(Ident, Expr, Punctuated<Expr, Token![,]>)>);

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
            #[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
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
                    let (#pk, #bump) = crayfish_program::pubkey::find_program_address(&[#seeds], &crate::ID);
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
                    let #pk = crayfish_program::pubkey::create_program_address(&[#seeds, &[#value]], &crate::ID)?;
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
                        if ident.to_string() == bump.to_string() {
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

        (
            checks,
            assign,
        )
    }
}

impl TryFrom<&Vec<Account>> for Bumps {
    type Error = syn::Error;

    fn try_from(accounts: &Vec<Account>) -> Result<Self, Self::Error> {
        Ok(Bumps(
            accounts
                .iter()
                .filter(|a| {
                    a.constraints.has_init()
                        && a.constraints.get_bump(&a.name).is_some()
                        && a.constraints.get_seeds().is_some()
                })
                .map(|a| {
                    (
                        a.name.clone(),
                        a.constraints.get_bump(&a.name).unwrap().clone(),
                        a.constraints.get_seeds().unwrap().clone(),
                    )
                })
                .collect(),
        ))
    }
}

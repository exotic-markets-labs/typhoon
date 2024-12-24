use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{spanned::Spanned, Attribute, Fields, Ident, Meta, Type},
};

pub struct PrimaryKey {
    pub name: Ident,
    pub ty: Type,
}

impl PrimaryKey {
    pub fn to_bytes_tokens(&self) -> TokenStream {
        let invalid_type_err =
            syn::Error::new(self.name.span(), "This type cannot be used as a key")
                .to_compile_error();
        let token_stream = if let Type::Path(path) = &self.ty {
            if let Some(ident) = path.path.get_ident() {
                let type_name = ident.to_string();
                match type_name.as_str() {
                    "Pubkey" => {
                        let name = &self.name;
                        quote! { #name.as_ref() }
                    }
                    "u64" => {
                        let name = &self.name;
                        quote! { #name.to_le_bytes().as_ref() }
                    }
                    "u32" => {
                        let name = &self.name;
                        quote! { #name.to_le_bytes().as_ref() }
                    }
                    "u16" => {
                        let name = &self.name;
                        quote! { #name.to_le_bytes().as_ref() }
                    }
                    "u8" => {
                        let name = &self.name;
                        quote! { #name.to_le_bytes().as_ref() }
                    }
                    _ => invalid_type_err,
                }
            } else {
                invalid_type_err
            }
        } else {
            syn::Error::new(self.name.span(), "This type cannot be used as a key")
                .to_compile_error()
        };
        token_stream
    }
}

pub struct PrimaryKeys(Vec<PrimaryKey>);

impl PrimaryKeys {
    pub fn split_for_impl(&self, account_name: &Ident) -> TokenStream {
        let parameters = self.0.iter().map(|k| {
            let name = &k.name;
            let ty = &k.ty;

            quote! { #name: &#ty }
        });
        let parameters_list = quote! { #(#parameters),* };
        let parameters_with_lifetime = self.0.iter().map(|k| {
            let name = &k.name;
            let ty = &k.ty;

            quote! { #name: &'a #ty }
        });
        let parameters_list_with_lifetime = quote! { #(#parameters_with_lifetime),* };

        let params_to_seed = self.0.iter().map(|k| k.to_bytes_tokens());
        let params_to_self_seed = params_to_seed.clone();
        let n_seeds = self.0.len() + 1;
        let n_seeds_with_bump = self.0.len() + 2;
        let self_seeds = quote! { #(self.#params_to_self_seed),* };
        let seeds = quote! { #(#params_to_seed),* };

        let lowercase_name = account_name.to_string().to_lowercase();
        let seeded_trait = quote! {
            impl #account_name {
                const BASE_SEED: &'static [u8] = #lowercase_name.as_bytes();

                pub fn seeds<'a>(&'a self) -> [&'a [u8]; #n_seeds] {
                    [Self::BASE_SEED, #self_seeds]
                }

                pub fn derive(#parameters_list) -> [&[u8]; #n_seeds] {
                    [Self::BASE_SEED, #seeds]
                }

                // TODO: use the bump stored in the account
                pub fn seeds_with_bump<'a>(&'a self, bump: &'a [u8]) -> [&'a [u8]; #n_seeds_with_bump] {
                    [Self::BASE_SEED, #self_seeds, bump]
                }

                pub fn derive_with_bump<'a>(#parameters_list_with_lifetime, bump: &'a [u8]) -> [&'a [u8]; #n_seeds_with_bump] {
                    [Self::BASE_SEED, #seeds, bump]
                }
            }
        };

        if n_seeds > 1 {
            seeded_trait
        } else {
            quote!()
        }
    }
}

impl TryFrom<&Fields> for PrimaryKeys {
    type Error = syn::Error;

    fn try_from(value: &Fields) -> Result<Self, syn::Error> {
        match value {
            Fields::Named(fields) => Ok(PrimaryKeys(
                fields
                    .named
                    .iter()
                    .filter_map(|f| {
                        let field = f.clone();
                        let keys = f
                            .attrs
                            .iter()
                            .filter(|a| match &a.meta {
                                Meta::Path(path) => {
                                    let ident = path.get_ident();

                                    ident.map_or(false, |ident| *ident == "key")
                                }
                                _ => false,
                            })
                            .collect::<Vec<&Attribute>>();
                        let key = keys.first();

                        if let (Some(_), Some(ident), ty) = (key, field.ident, field.ty) {
                            Some((ident, ty))
                        } else {
                            None
                        }
                    })
                    .map(|(ident, ty)| PrimaryKey { name: ident, ty })
                    .collect::<Vec<PrimaryKey>>(),
            )),
            _ => Err(syn::Error::new(
                value.span(),
                "Only named fields are currently handled",
            )),
        }
    }
}

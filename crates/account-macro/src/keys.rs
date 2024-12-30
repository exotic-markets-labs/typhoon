use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{spanned::Spanned, Fields, Ident, Meta, Type},
};

pub struct PrimaryKey {
    pub name: Ident,
    pub ty: Type,
}

impl PrimaryKey {
    pub fn to_bytes_tokens(&self) -> TokenStream {
        let name = &self.name;

        match &self.ty {
            Type::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    match ident.to_string().as_str() {
                        "Pubkey" => quote! { #name.as_ref() },
                        "u64" | "u32" | "u16" | "u8" => quote! { #name.to_le_bytes().as_ref() },
                        _ => syn::Error::new(self.name.span(), "This type cannot be used as a key")
                            .to_compile_error(),
                    }
                } else {
                    syn::Error::new(self.name.span(), "This type cannot be used as a key")
                        .to_compile_error()
                }
            }
            _ => syn::Error::new(self.name.span(), "This type cannot be used as a key")
                .to_compile_error(),
        }
    }
}

pub struct PrimaryKeys(Vec<PrimaryKey>);

impl PrimaryKeys {
    pub fn split_for_impl(&self, account_name: &Ident) -> TokenStream {
        let has_keys = !self.0.is_empty();
        let n_seeds = self.0.len() + 1;
        let n_seeds_with_bump = n_seeds + 1;

        if !has_keys {
            return quote!();
        }

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

        let params_to_seed: Vec<_> = self.0.iter().map(|k| k.to_bytes_tokens()).collect();
        let self_seeds = quote! { #(self.#params_to_seed),* };
        let seeds = quote! { #(#params_to_seed),* };

        let lowercase_name = account_name.to_string().to_lowercase();

        quote! {
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
        }
    }
}

impl TryFrom<&Fields> for PrimaryKeys {
    type Error = syn::Error;

    fn try_from(value: &Fields) -> Result<Self, syn::Error> {
        match value {
            Fields::Named(fields) => {
                let mut primary_keys = Vec::new();

                for field in fields.named.iter() {
                    let has_key = field.attrs.iter().any(|attr| {
                        if let Meta::Path(path) = &attr.meta {
                            path.get_ident().map_or(false, |ident| *ident == "key")
                        } else {
                            false
                        }
                    });

                    if has_key {
                        if let Some(ident) = &field.ident {
                            primary_keys.push(PrimaryKey {
                                name: ident.clone(),
                                ty: field.ty.clone(),
                            });
                        }
                    }
                }

                Ok(PrimaryKeys(primary_keys))
            }
            _ => Err(syn::Error::new(
                value.span(),
                "Only named fields are currently handled",
            )),
        }
    }
}

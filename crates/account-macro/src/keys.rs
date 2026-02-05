use {
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{spanned::Spanned, Fields, Ident, Meta, Type},
};

pub struct PrimaryKey {
    pub name: Ident,
    pub ty: Type,
}

impl PrimaryKey {
    fn type_ident(&self) -> Option<&Ident> {
        match &self.ty {
            Type::Path(path) => path.path.get_ident(),
            _ => None,
        }
    }

    /// Type for the field in the seeds holder struct.
    fn seeds_field_ty(&self) -> Result<TokenStream, TokenStream> {
        let Some(ident) = self.type_ident() else {
            return Err(
                syn::Error::new(self.name.span(), "This type cannot be used as a key")
                    .to_compile_error(),
            );
        };
        match ident.to_string().as_str() {
            "Address" => Ok(quote! { &'a [u8] }),
            "u64" => Ok(quote! { [u8; 8] }),
            "u32" => Ok(quote! { [u8; 4] }),
            "u16" => Ok(quote! { [u8; 2] }),
            "u8" => Ok(quote! { [u8; 1] }),
            _ => Err(
                syn::Error::new(self.name.span(), "This type cannot be used as a key")
                    .to_compile_error(),
            ),
        }
    }

    /// Expression to construct the seeds field from `self.field`.
    fn self_init_expr(&self) -> TokenStream {
        let name = &self.name;
        match self.type_ident().map(|i| i.to_string()) {
            Some(ref s) if s == "Address" => quote! { self.#name.as_ref() },
            _ => quote! { self.#name.to_le_bytes() },
        }
    }

    /// Expression to construct the seeds field from a derive parameter.
    fn derive_init_expr(&self) -> TokenStream {
        let name = &self.name;
        match self.type_ident().map(|i| i.to_string()) {
            Some(ref s) if s == "Address" => quote! { #name.as_ref() },
            _ => quote! { #name.to_le_bytes() },
        }
    }

    /// Expression to get `&[u8]` from the seeds holder struct field.
    fn seed_ref_expr(&self) -> TokenStream {
        let name = &self.name;
        match self.type_ident().map(|i| i.to_string()) {
            Some(ref s) if s == "Address" => quote! { self.#name },
            _ => quote! { self.#name.as_ref() },
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

        let seeds_struct_name = format_ident!("{}Seeds", account_name);

        let struct_fields = self.0.iter().map(|k| {
            let name = &k.name;
            let ty = match k.seeds_field_ty() {
                Ok(ty) => ty,
                Err(err) => return err,
            };
            quote! { #name: #ty }
        });

        let self_init_fields = self.0.iter().map(|k| {
            let name = &k.name;
            let expr = k.self_init_expr();
            quote! { #name: #expr }
        });

        let derive_init_fields = self.0.iter().map(|k| {
            let name = &k.name;
            let expr = k.derive_init_expr();
            quote! { #name: #expr }
        });

        let seed_refs: Vec<_> = self.0.iter().map(|k| k.seed_ref_expr()).collect();

        let parameters_with_lifetime = self.0.iter().map(|k| {
            let name = &k.name;
            let ty = &k.ty;
            quote! { #name: &'a #ty }
        });
        let parameters_list_with_lifetime = quote! { #(#parameters_with_lifetime),* };

        let lowercase_name = account_name.to_string().to_lowercase();

        quote! {
            pub struct #seeds_struct_name<'a> {
                #(#struct_fields),*
            }

            impl<'a> #seeds_struct_name<'a> {
                pub fn as_seeds(&'a self) -> [&'a [u8]; #n_seeds] {
                    [#account_name::BASE_SEED, #(#seed_refs),*]
                }

                pub fn seeds_with_bump(&'a self, bump: &'a [u8]) -> [&'a [u8]; #n_seeds_with_bump] {
                    [#account_name::BASE_SEED, #(#seed_refs),*, bump]
                }

                pub fn signer_seeds_with_bump(&'a self, bump: &'a [u8]) -> [Seed<'a>; #n_seeds_with_bump] {
                    seeds!(#account_name::BASE_SEED, #(#seed_refs),*, bump)
                }
            }

            impl #account_name {
                const BASE_SEED: &'static [u8] = #lowercase_name.as_bytes();

                pub fn seeds(&self) -> #seeds_struct_name<'_> {
                    #seeds_struct_name {
                        #(#self_init_fields),*
                    }
                }

                pub fn derive<'a>(#parameters_list_with_lifetime) -> #seeds_struct_name<'a> {
                    #seeds_struct_name {
                        #(#derive_init_fields),*
                    }
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
                            path.get_ident().is_some_and(|ident| *ident == "key")
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

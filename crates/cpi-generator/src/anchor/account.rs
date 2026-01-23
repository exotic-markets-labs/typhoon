use {
    crate::{
        anchor::{gen_docs, gen_type},
        idl::{
            Account, DefinedFields, EnumVariant, Repr, ReprModifier, Serialization, Type, TypeDef,
            TypeDefTy,
        },
    },
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote},
    std::collections::HashMap,
    syn::Ident,
};

pub fn gen_accounts(accounts: &[Account], types: &[TypeDef]) -> proc_macro2::TokenStream {
    let mut types: HashMap<String, TokenStream> = types
        .iter()
        .map(|ty| (ty.name.to_string(), gen_defined_type(ty)))
        .collect();

    for account in accounts {
        let ident = format_ident!("{}", account.name);
        let discriminator = &account.discriminator;
        let traits_impl = quote! {
            impl Owner for #ident {
                const OWNER: Address = PROGRAM_ID;
            }

            impl Discriminator for #ident {
                const DISCRIMINATOR: &'static [u8] = &[#(#discriminator),*];
            }
        };
        if let Some(ty) = &account.ty {
            let type_def = TypeDef {
                name: account.name.to_owned(),
                ty: ty.to_owned(),
                ..Default::default()
            };
            let ty = gen_defined_type(&type_def);
            types.insert(
                account.name.clone(),
                quote! {
                    #ty
                    #traits_impl
                },
            );
        } else {
            let ty = types.get_mut(&account.name).unwrap();
            ty.extend(traits_impl);
        }
    }

    let types = types.values();

    quote! {
        #(#types)*
    }
}

fn gen_defined_type(ty: &TypeDef) -> proc_macro2::TokenStream {
    let ident = format_ident!("{}", ty.name);
    let repr = ty.repr.as_ref().map(gen_repr);
    let docs = gen_docs(&ty.docs);
    let derive = gen_serialization(&ty.serialization);
    let item = match &ty.ty {
        TypeDefTy::Struct { fields } => gen_struct(&ident, fields),
        TypeDefTy::Enum { variants } => gen_enum(&ident, variants),
        TypeDefTy::Type { alias } => gen_type_alias(&ident, alias),
    };

    quote! {
        #docs
        #derive
        #repr
        #item
    }
}

fn gen_struct(ident: &Ident, fields: &Option<DefinedFields>) -> proc_macro2::TokenStream {
    match fields {
        Some(struct_fields) => match struct_fields {
            DefinedFields::Named(f) => {
                let fields = f.iter().map(|el| {
                    let docs = gen_docs(&el.docs);
                    let ident = Ident::new(&el.name, Span::call_site());
                    let ty = gen_type(&el.ty);

                    quote! {
                        #docs
                        pub #ident: #ty,
                    }
                });
                quote! {
                    pub struct #ident {
                        #(#fields)*
                    }
                }
            }
            DefinedFields::Tuple(f) => {
                let fields = f.iter().map(|el| {
                    let ty = gen_type(el);
                    quote!(#ty)
                });
                quote! {
                    pub struct #ident(#(#fields),*)
                }
            }
        },
        None => quote!(pub struct #ident;),
    }
}

fn gen_enum(ident: &Ident, variants: &[EnumVariant]) -> proc_macro2::TokenStream {
    let fields = variants.iter().map(|el| {
        let variant_ident = Ident::new(&el.name, Span::call_site());
        if let Some(ref f) = el.fields {
            match f {
                DefinedFields::Named(f) => {
                    let fields = f.iter().map(|el| {
                        let docs = gen_docs(&el.docs);
                        let ident = Ident::new(&el.name, Span::call_site());
                        let ty = gen_type(&el.ty);

                        quote! {
                            #docs
                            #ident: #ty,
                        }
                    });
                    quote! {
                        #variant_ident {
                            #(#fields)*
                        }
                    }
                }
                DefinedFields::Tuple(f) => {
                    let fields = f.iter().map(|el| {
                        let ty = gen_type(el);
                        quote!(#ty)
                    });
                    quote! {
                        #variant_ident(#(#fields),*)
                    }
                }
            }
        } else {
            quote!(#variant_ident)
        }
    });

    quote! {
        pub enum #ident {
            #(#fields),*
        }
    }
}

fn gen_type_alias(ident: &Ident, alias: &Type) -> proc_macro2::TokenStream {
    let ty = gen_type(alias);
    quote!(pub type #ident = #ty;)
}

fn gen_repr(r: &Repr) -> proc_macro2::TokenStream {
    let gen_repr_with_modifiers = |repr_type: &str, modifier: &ReprModifier| {
        let ident = Ident::new(repr_type, Span::call_site());
        let mut attrs = vec![quote!(#ident)];

        if modifier.packed {
            attrs.push(quote!(packed));
        }
        if let Some(size) = modifier.align {
            attrs.push(quote!(align(#size)));
        }

        quote!(#[repr(#(#attrs),*)])
    };

    match r {
        Repr::Rust(modifier) => gen_repr_with_modifiers("Rust", modifier),
        Repr::C(modifier) => gen_repr_with_modifiers("C", modifier),
        Repr::Transparent => quote!(#[repr(transparent)]),
    }
}

fn gen_serialization(serialization: &Serialization) -> proc_macro2::TokenStream {
    match serialization {
        Serialization::Borsh => {
            quote!(#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)])
        }
        Serialization::BytemuckUnsafe | Serialization::Bytemuck => {
            quote!(#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)])
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::idl::Field, quote::quote};

    #[test]
    fn test_gen_repr_rust() {
        let repr = Repr::Rust(ReprModifier {
            packed: true,
            align: Some(4),
        });
        let result = gen_repr(&repr).to_string();
        assert_eq!(
            result,
            quote!(#[repr(Rust, packed, align(4usize))]).to_string()
        );
    }

    #[test]
    fn test_gen_repr_c() {
        let repr = Repr::C(ReprModifier {
            packed: false,
            align: Some(8),
        });
        let result = gen_repr(&repr).to_string();
        assert_eq!(result, quote!(#[repr(C, align(8usize))]).to_string());
    }

    #[test]
    fn test_gen_repr_transparent() {
        let repr = Repr::Transparent;
        let result = gen_repr(&repr).to_string();
        assert_eq!(result, quote!(#[repr(transparent)]).to_string());
    }

    #[test]
    fn test_gen_repr_no_modifiers() {
        let repr = Repr::Rust(ReprModifier {
            packed: false,
            align: None,
        });
        let result = gen_repr(&repr).to_string();
        assert_eq!(result, quote!(#[repr(Rust)]).to_string());
    }

    #[test]
    fn test_gen_serialization_borsh() {
        let result = gen_serialization(&Serialization::Borsh).to_string();
        assert_eq!(
            result,
            quote!(#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]).to_string()
        );
    }

    #[test]
    fn test_gen_serialization_bytemuck() {
        let result = gen_serialization(&Serialization::Bytemuck).to_string();
        assert_eq!(
            result,
            quote!(#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]).to_string()
        );
    }

    #[test]
    fn test_gen_struct_named() {
        let ident = Ident::new("TestStruct", Span::call_site());
        let fields = DefinedFields::Named(vec![Field {
            name: "field1".to_string(),
            docs: vec!["Test doc".to_string()],
            ty: Type::U64,
        }]);
        let result = gen_struct(&ident, &Some(fields)).to_string();
        assert_eq!(
            result,
            quote! {
                pub struct TestStruct {
                    #[doc = " Test doc"]
                    pub field1: u64,
                }
            }
            .to_string()
        );
    }

    #[test]
    fn test_gen_struct_tuple() {
        let ident = Ident::new("TestStruct", Span::call_site());
        let fields = DefinedFields::Tuple(vec![Type::U64, Type::Bool]);
        let result = gen_struct(&ident, &Some(fields)).to_string();
        assert_eq!(result, quote!(pub struct TestStruct(u64, bool)).to_string());
    }

    #[test]
    fn test_gen_struct_empty() {
        let ident = Ident::new("TestStruct", Span::call_site());
        let result = gen_struct(&ident, &None).to_string();
        assert_eq!(
            result,
            quote!(
                pub struct TestStruct;
            )
            .to_string()
        );
    }

    #[test]
    fn test_gen_enum() {
        let ident = Ident::new("TestEnum", Span::call_site());
        let variants = vec![
            EnumVariant {
                name: "Variant1".to_string(),
                fields: None,
            },
            EnumVariant {
                name: "Variant2".to_string(),
                fields: Some(DefinedFields::Named(vec![Field {
                    name: "field1".to_string(),
                    docs: vec![],
                    ty: Type::U64,
                }])),
            },
            EnumVariant {
                name: "Variant3".to_string(),
                fields: Some(DefinedFields::Tuple(vec![Type::Bool, Type::U64])),
            },
        ];
        let result = gen_enum(&ident, &variants).to_string();
        assert_eq!(
            result,
            quote! {
                pub enum TestEnum {
                    Variant1,
                    Variant2 {
                        field1: u64,
                    },
                    Variant3(bool, u64)
                }
            }
            .to_string()
        );
    }

    #[test]
    fn test_gen_type_alias() {
        let ident = Ident::new("TestAlias", Span::call_site());
        let alias = Type::U64;
        let result = gen_type_alias(&ident, &alias).to_string();
        assert_eq!(
            result,
            quote!(
                pub type TestAlias = u64;
            )
            .to_string()
        );
    }
}

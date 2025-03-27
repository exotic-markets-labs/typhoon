use {
    crate::anchor::{gen_docs, gen_type},
    anchor_lang_idl_spec::{
        Idl, IdlDefinedFields, IdlEnumVariant, IdlRepr, IdlReprModifier, IdlSerialization, IdlType,
        IdlTypeDefTy,
    },
    heck::ToUpperCamelCase,
    proc_macro2::Span,
    quote::quote,
    syn::Ident,
};

pub fn gen_accounts(idl: &Idl) -> proc_macro2::TokenStream {
    let program_name = &idl.metadata.name.to_upper_camel_case();
    let program_ident = Ident::new(&format!("{program_name}Program"), Span::call_site());
    let types = idl.types.iter().map(|i| {
        let ident = Ident::new(&i.name, Span::call_site());
        let repr = i.repr.as_ref().map(gen_repr);
        let docs = gen_docs(&i.docs);
        let derive = gen_serialization(&i.serialization);
        let item = match &i.ty {
            IdlTypeDefTy::Struct { fields } => gen_struct(&ident, fields),
            IdlTypeDefTy::Enum { variants } => gen_enum(&ident, variants),
            IdlTypeDefTy::Type { alias } => gen_type_alias(&ident, alias),
        };
        let maybe_owner = idl
            .accounts
            .iter()
            .find(|acc| acc.name == i.name)
            .map(|acc| {
                let discriminator = &acc.discriminator;

                quote! {
                    impl Owner for #ident {
                        const OWNER: Pubkey = #program_ident::ID;
                    }

                    impl Discriminator for #ident {
                        const DISCRIMINATOR: &'static [u8] = &[#(#discriminator),*];
                    }
                }
            });
        // TODO generics

        quote! {
            #docs
            #derive
            #repr
            #item
            #maybe_owner
        }
    });

    quote! {
        #(#types)*
    }
}

fn gen_struct(ident: &Ident, fields: &Option<IdlDefinedFields>) -> proc_macro2::TokenStream {
    match fields {
        Some(struct_fields) => match struct_fields {
            IdlDefinedFields::Named(f) => {
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
            IdlDefinedFields::Tuple(f) => {
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

fn gen_enum(ident: &Ident, variants: &[IdlEnumVariant]) -> proc_macro2::TokenStream {
    let fields = variants.iter().map(|el| {
        let variant_ident = Ident::new(&el.name, Span::call_site());
        if let Some(ref f) = el.fields {
            match f {
                IdlDefinedFields::Named(f) => {
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
                        #variant_ident {
                            #(#fields)*
                        }
                    }
                }
                IdlDefinedFields::Tuple(f) => {
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

fn gen_type_alias(ident: &Ident, alias: &IdlType) -> proc_macro2::TokenStream {
    let ty = gen_type(alias);
    quote!(pub type #ident = #ty;)
}

fn gen_repr(r: &IdlRepr) -> proc_macro2::TokenStream {
    let gen_repr_with_modifiers = |repr_type: &str, modifier: &IdlReprModifier| {
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
        IdlRepr::Rust(modifier) => gen_repr_with_modifiers("Rust", modifier),
        IdlRepr::C(modifier) => gen_repr_with_modifiers("C", modifier),
        IdlRepr::Transparent => quote!(#[repr(transparent)]),
        _ => unimplemented!(),
    }
}

fn gen_serialization(serialization: &IdlSerialization) -> proc_macro2::TokenStream {
    match serialization {
        IdlSerialization::Borsh => {
            quote!(#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)])
        }
        IdlSerialization::BytemuckUnsafe | IdlSerialization::Bytemuck => {
            quote!(#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)])
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use {super::*, anchor_lang_idl_spec::IdlField, quote::quote};

    #[test]
    fn test_gen_repr_rust() {
        let repr = IdlRepr::Rust(IdlReprModifier {
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
        let repr = IdlRepr::C(IdlReprModifier {
            packed: false,
            align: Some(8),
        });
        let result = gen_repr(&repr).to_string();
        assert_eq!(result, quote!(#[repr(C, align(8usize))]).to_string());
    }

    #[test]
    fn test_gen_repr_transparent() {
        let repr = IdlRepr::Transparent;
        let result = gen_repr(&repr).to_string();
        assert_eq!(result, quote!(#[repr(transparent)]).to_string());
    }

    #[test]
    fn test_gen_repr_no_modifiers() {
        let repr = IdlRepr::Rust(IdlReprModifier {
            packed: false,
            align: None,
        });
        let result = gen_repr(&repr).to_string();
        assert_eq!(result, quote!(#[repr(Rust)]).to_string());
    }

    #[test]
    fn test_gen_serialization_borsh() {
        let result = gen_serialization(&IdlSerialization::Borsh).to_string();
        assert_eq!(
            result,
            quote!(#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]).to_string()
        );
    }

    #[test]
    fn test_gen_serialization_bytemuck() {
        let result = gen_serialization(&IdlSerialization::Bytemuck).to_string();
        assert_eq!(
            result,
            quote!(#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]).to_string()
        );
    }

    #[test]
    fn test_gen_struct_named() {
        let ident = Ident::new("TestStruct", Span::call_site());
        let fields = IdlDefinedFields::Named(vec![IdlField {
            name: "field1".to_string(),
            docs: vec!["Test doc".to_string()],
            ty: IdlType::U64,
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
        let fields = IdlDefinedFields::Tuple(vec![IdlType::U64, IdlType::Bool]);
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
            IdlEnumVariant {
                name: "Variant1".to_string(),
                fields: None,
            },
            IdlEnumVariant {
                name: "Variant2".to_string(),
                fields: Some(IdlDefinedFields::Named(vec![IdlField {
                    name: "field1".to_string(),
                    docs: vec![],
                    ty: IdlType::U64,
                }])),
            },
            IdlEnumVariant {
                name: "Variant3".to_string(),
                fields: Some(IdlDefinedFields::Tuple(vec![IdlType::Bool, IdlType::U64])),
            },
        ];
        let result = gen_enum(&ident, &variants).to_string();
        assert_eq!(
            result,
            quote! {
                pub enum TestEnum {
                    Variant1,
                    Variant2 {
                        pub field1: u64,
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
        let alias = IdlType::U64;
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

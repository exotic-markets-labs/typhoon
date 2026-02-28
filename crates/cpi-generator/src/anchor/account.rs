use {
    crate::{
        anchor::{gen_docs, gen_type},
        idl::{
            Account, DefinedFields, EnumVariant, Field, Repr, ReprModifier, Serialization, Type,
            TypeDef, TypeDefTy,
        },
    },
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote},
    std::collections::{hash_map::Entry, HashMap},
    syn::Ident,
};

pub fn gen_accounts(accounts: &[Account], types: &[TypeDef]) -> TokenStream {
    let serialization_map: HashMap<&str, &Serialization> = types
        .iter()
        .map(|ty| (ty.name.as_str(), &ty.serialization))
        .collect();

    let mut generated_types_map: HashMap<String, TokenStream> = types
        .iter()
        .map(|ty| (ty.name.to_string(), gen_defined_type(ty)))
        .collect();

    for account in accounts {
        let traits_impl = gen_account_traits(account, serialization_map.get(account.name.as_str()));

        match (&account.ty, generated_types_map.entry(account.name.clone())) {
            (Some(ty), entry) => {
                let type_def = TypeDef {
                    name: account.name.clone(),
                    ty: ty.clone(),
                    ..Default::default()
                };
                let generated_type = gen_defined_type(&type_def);
                entry.insert_entry(quote! {
                    #generated_type
                    #traits_impl
                });
            }
            (None, Entry::Occupied(mut entry)) => {
                entry.get_mut().extend(traits_impl);
            }
            (None, Entry::Vacant(_)) => {
                panic!(
                    "Account '{}' has no type definition and no matching type in types array",
                    account.name
                );
            }
        }
    }

    let generated_types = generated_types_map.into_values();
    quote!(#(#generated_types)*)
}

fn gen_account_traits(account: &Account, serialization: Option<&&Serialization>) -> TokenStream {
    let ident = format_ident!("{}", account.name);
    let discriminator = &account.discriminator;

    let strategy = match serialization.unwrap_or(&&Serialization::Borsh) {
        Serialization::Borsh | Serialization::Custom(_) => quote!(BorshStrategy),
        Serialization::Bytemuck | Serialization::BytemuckUnsafe => quote!(BytemuckStrategy),
    };

    quote! {
        impl CheckOwner for #ident {
            #[inline(always)]
            fn owned_by(owner: &Address) -> bool {
                address_eq(owner, &PROGRAM_ID)
            }
        }

        impl Discriminator for #ident {
            const DISCRIMINATOR: &'static [u8] = &[#(#discriminator),*];
        }

        impl AccountStrategy for #ident {
            type Strategy = #strategy;
        }
    }
}

fn gen_defined_type(ty: &TypeDef) -> TokenStream {
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

fn gen_struct(ident: &Ident, fields: &Option<DefinedFields>) -> TokenStream {
    match fields {
        Some(DefinedFields::Named(f)) => {
            let fields = f.iter().map(|field| gen_field(field, true));
            quote!(pub struct #ident { #(#fields)* })
        }
        Some(DefinedFields::Tuple(f)) => {
            let fields = gen_tuple_fields(f);
            quote!(pub struct #ident(#(#fields),*))
        }
        None => quote!(pub struct #ident;),
    }
}

fn gen_enum(ident: &Ident, variants: &[EnumVariant]) -> TokenStream {
    let variant_tokens = variants.iter().map(gen_enum_variant);
    quote!(pub enum #ident { #(#variant_tokens),* })
}

fn gen_enum_variant(variant: &EnumVariant) -> TokenStream {
    let variant_ident = Ident::new(&variant.name, Span::call_site());

    match &variant.fields {
        Some(DefinedFields::Named(f)) => {
            let fields = f.iter().map(|field| gen_field(field, false));
            quote!(#variant_ident { #(#fields)* })
        }
        Some(DefinedFields::Tuple(f)) => {
            let fields = gen_tuple_fields(f);
            quote!(#variant_ident(#(#fields),*))
        }
        None => quote!(#variant_ident),
    }
}

fn gen_type_alias(ident: &Ident, alias: &Type) -> TokenStream {
    let ty = gen_type(alias);
    quote!(pub type #ident = #ty;)
}

fn gen_field(field: &Field, public: bool) -> TokenStream {
    let docs = gen_docs(&field.docs);
    let ident = Ident::new(&field.name, Span::call_site());
    let ty = gen_type(&field.ty);
    if public {
        quote!(#docs pub #ident: #ty,)
    } else {
        quote!(#docs #ident: #ty,)
    }
}

fn gen_tuple_fields(types: &[Type]) -> impl Iterator<Item = syn::Type> + '_ {
    types.iter().map(gen_type)
}

fn gen_repr(r: &Repr) -> TokenStream {
    match r {
        Repr::Rust(modifier) => gen_repr_with_modifiers("Rust", modifier),
        Repr::C(modifier) => gen_repr_with_modifiers("C", modifier),
        Repr::Transparent => quote!(#[repr(transparent)]),
    }
}

fn gen_repr_with_modifiers(repr_type: &str, modifier: &ReprModifier) -> TokenStream {
    let ident = Ident::new(repr_type, Span::call_site());
    let packed = modifier.packed.then(|| quote!(packed));
    let align = modifier.align.map(|size| quote!(align(#size)));
    let attrs = [Some(quote!(#ident)), packed, align].into_iter().flatten();
    quote!(#[repr(#(#attrs),*)])
}

fn gen_serialization(serialization: &Serialization) -> TokenStream {
    match serialization {
        Serialization::Borsh | Serialization::Custom(_) => {
            quote!(#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)])
        }
        Serialization::Bytemuck | Serialization::BytemuckUnsafe => {
            quote!(#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)])
        }
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

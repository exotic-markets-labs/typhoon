use {
    anchor_lang_idl_spec::{IdlArrayLen, IdlGenericArg, IdlType},
    proc_macro2::Span,
    quote::quote,
    syn::{parse_quote, Ident, Type},
};

pub fn gen_type(idl_ty: &IdlType) -> Type {
    match idl_ty {
        IdlType::Bool => parse_quote!(bool),
        IdlType::U8 => parse_quote!(u8),
        IdlType::I8 => parse_quote!(i8),
        IdlType::U16 => parse_quote!(u16),
        IdlType::I16 => parse_quote!(i16),
        IdlType::U32 => parse_quote!(u32),
        IdlType::I32 => parse_quote!(i32),
        IdlType::F32 => parse_quote!(f32),
        IdlType::U64 => parse_quote!(u64),
        IdlType::I64 => parse_quote!(i64),
        IdlType::F64 => parse_quote!(f64),
        IdlType::U128 => parse_quote!(u128),
        IdlType::I128 => parse_quote!(i128),
        IdlType::Bytes => parse_quote!(Vec<u8>),
        IdlType::String => parse_quote!(String),
        IdlType::Pubkey => parse_quote!(Pubkey),
        IdlType::Option(inner) => {
            let ty = gen_type(inner);
            parse_quote!(Option<#ty>)
        }
        IdlType::Vec(inner) => {
            let ty = gen_type(inner);
            parse_quote!(Vec<#ty>)
        }
        IdlType::Defined { name, generics } => {
            let ident = Ident::new(name, Span::call_site());
            if generics.is_empty() {
                parse_quote!(#ident)
            } else {
                let generic_types = generics.iter().map(|g| match g {
                    IdlGenericArg::Type { ty } => gen_type(ty),
                    IdlGenericArg::Const { value } => parse_quote!(#value),
                });
                parse_quote!(#ident<#(#generic_types),*>)
            }
        }
        IdlType::Array(inner, len) => {
            let ty = gen_type(inner);
            let size = match len {
                IdlArrayLen::Generic(size) => quote!(#size),
                IdlArrayLen::Value(size) => quote!(#size),
            };
            parse_quote!([#ty; #size])
        }
        IdlType::U256 | IdlType::I256 | IdlType::Generic(_) => unimplemented!(),
        _ => unimplemented!(),
    }
}

pub fn gen_type_ref(idl_ty: &IdlType) -> Type {
    match idl_ty {
        IdlType::Bool => parse_quote!(bool),
        IdlType::U8 => parse_quote!(u8),
        IdlType::I8 => parse_quote!(i8),
        IdlType::U16 => parse_quote!(u16),
        IdlType::I16 => parse_quote!(i16),
        IdlType::U32 => parse_quote!(u32),
        IdlType::I32 => parse_quote!(i32),
        IdlType::F32 => parse_quote!(f32),
        IdlType::U64 => parse_quote!(u64),
        IdlType::I64 => parse_quote!(i64),
        IdlType::F64 => parse_quote!(f64),
        IdlType::U128 => parse_quote!(u128),
        IdlType::I128 => parse_quote!(i128),
        IdlType::Bytes => parse_quote!(&'a [u8]),
        IdlType::String => parse_quote!(&'a str),
        IdlType::Pubkey => parse_quote!(&'a Pubkey),
        IdlType::Option(inner) => {
            let ty = gen_type_ref(inner);
            parse_quote!(Option<#ty>)
        }
        IdlType::Vec(inner) | IdlType::Array(inner, _) => {
            let ty = gen_type_ref(inner);
            parse_quote!(&'a [#ty])
        }
        IdlType::Defined { name, generics } => {
            let ident = Ident::new(name, Span::call_site());
            if generics.is_empty() {
                parse_quote!(&'a #ident)
            } else {
                let generic_types = generics.iter().map(|g| match g {
                    IdlGenericArg::Type { ty } => gen_type_ref(ty),
                    IdlGenericArg::Const { value } => parse_quote!(#value),
                });
                parse_quote!(&'a #ident<#(#generic_types),*>)
            }
        }
        IdlType::U256 | IdlType::I256 | IdlType::Generic(_) => unimplemented!(),
        _ => unimplemented!(),
    }
}

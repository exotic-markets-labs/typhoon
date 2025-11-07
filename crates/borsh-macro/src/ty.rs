use {
    proc_macro2::TokenStream as TokenStream2,
    quote::{format_ident, quote},
    syn::{AngleBracketedGenericArguments, Expr, GenericArgument, Ident, PathArguments, Type},
};

pub enum SupportedType {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    String,
    Pubkey,
    Option(Box<SupportedType>),
    Vec(Box<SupportedType>),
    Array(Box<SupportedType>, Expr),
    Defined(String),
}

impl SupportedType {
    pub fn gen_read_expr(&self, is_inner: bool) -> (TokenStream2, TokenStream2) {
        match self {
            SupportedType::Bool => {
                let add_offset = is_inner.then_some(quote!(offset += 1;));
                (
                    quote! {
                        let val = <bool as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(bool),
                )
            }
            SupportedType::U8 => {
                let add_offset = is_inner.then_some(quote!(offset += 1;));
                (
                    quote! {
                        let val = <u8 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(u8),
                )
            }
            SupportedType::I8 => {
                let add_offset = is_inner.then_some(quote!(offset += 1;));
                (
                    quote! {
                       let val = <i8 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                       #add_offset
                       val
                    },
                    quote!(i8),
                )
            }
            SupportedType::U16 => {
                let add_offset = is_inner.then_some(quote!(offset += 2;));
                (
                    quote! {
                        let val = <u16 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(u16),
                )
            }
            SupportedType::I16 => {
                let add_offset = is_inner.then_some(quote!(offset += 2;));
                (
                    quote! {
                        let val = <i16 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(i16),
                )
            }
            SupportedType::U32 => {
                let add_offset = is_inner.then_some(quote!(offset += 4;));
                (
                    quote! {
                        let val = <u32 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(u32),
                )
            }
            SupportedType::I32 => {
                let add_offset = is_inner.then_some(quote!(offset += 4;));
                (
                    quote! {
                        let val = <i32 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..])>;
                        #add_offset
                        val
                    },
                    quote!(i32),
                )
            }
            SupportedType::U64 => {
                let add_offset = is_inner.then_some(quote!(offset += 8;));
                (
                    quote! {
                        let val = <u64 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(u64),
                )
            }
            SupportedType::I64 => {
                let add_offset = is_inner.then_some(quote!(offset += 8;));
                (
                    quote! {
                        let val = <i64 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(i64),
                )
            }
            SupportedType::U128 => {
                let add_offset = is_inner.then_some(quote!(offset += 16;));
                (
                    quote! {
                        let val = <u128 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(u128),
                )
            }
            SupportedType::I128 => {
                let add_offset = is_inner.then_some(quote!(offset += 16;));
                (
                    quote! {
                        let val = <i128 as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(i128),
                )
            }
            SupportedType::String => {
                let add_offset = is_inner.then_some(quote!(offset += len;));
                (
                    quote! {
                        let len = u32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap()) as usize;
                        offset += 4;
                        let val = core::str::from_utf8(&self.0[offset..(offset + len)]).unwrap();
                        #add_offset
                        val
                    },
                    quote!(&str),
                )
            }
            SupportedType::Pubkey => {
                let add_offset = is_inner.then_some(quote!(offset += 32;));
                (
                    quote! {
                        let val = <&Pubkey as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(&Pubkey),
                )
            }
            SupportedType::Option(supported_type) => {
                let (_block, output_ty) = supported_type.gen_read_expr(true);

                (quote!(), quote!(Option<#output_ty>))
            }
            SupportedType::Vec(supported_type) => {
                if is_inner {
                    unimplemented!("Nested vec in no alloc env.")
                }
                let (_block, output_ty) = supported_type.gen_read_expr(true);
                (
                    quote! {
                        <BorshVector<#output_ty> as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..])
                    },
                    quote!(BorshVector<#output_ty>),
                )
            }
            SupportedType::Array(supported_type, expr) => {
                if is_inner {
                    unimplemented!("Nested array in no alloc env.")
                }
                let (block, output_ty) = supported_type.gen_read_expr(true);
                (
                    quote! {
                        let mut buffer: [core::mem::MaybeUninit<#output_ty>; #expr] = [const { core::mem::MaybeUninit::uninit() }; #expr];
                        for i in 0..#expr {
                            let val = { #block };
                            buffer[i].write(val);
                        }
                        buffer.map(|el| unsafe { el.assume_init() })
                    },
                    quote!([#output_ty; #expr]),
                )
            }
            SupportedType::Defined(name) => {
                let add_offset = is_inner.then_some(quote!(offset += val.total_len();));
                let name = format_ident!("{name}");
                (
                    quote! {
                        let val: &#name = <&#name as ::typhoon_borsh::BorshAccessor>::convert(&self.0[offset..]);
                        #add_offset
                        val
                    },
                    quote!(&#name),
                )
            }
        }
    }

    pub fn read_len_expr(&self, name: &Ident) -> TokenStream2 {
        match self {
            SupportedType::Bool | SupportedType::U8 | SupportedType::I8 => quote!(offset += 1;),
            SupportedType::U16 | SupportedType::I16 => quote!(offset += 2;),
            SupportedType::U32 | SupportedType::I32 => quote!(offset += 4;),
            SupportedType::U64 | SupportedType::I64 => quote!(offset += 8;),
            SupportedType::U128 | SupportedType::I128 => quote!(offset += 16;),
            SupportedType::String => quote! {
                let len = u32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap()) as usize;
                offset += 4 + len;
            },
            SupportedType::Pubkey => quote!(offset += 32;),
            SupportedType::Option(supported_type) => {
                let inner_len = supported_type.read_len_expr(name);
                quote! {
                    let is_some = self.0[offset] == 1;
                    offset += 1;
                    if is_some {
                        #inner_len;
                    }
                }
            }
            SupportedType::Vec(supported_type) => {
                let inner_len = supported_type.read_len_expr(name);
                quote! {
                    let len = u32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap()) as usize;
                    offset += 4;
                    for _ in 0..len {
                        #inner_len
                    }
                }
            }
            SupportedType::Array(supported_type, expr) => {
                let inner_len = supported_type.read_len_expr(name);
                quote! {
                    for _ in 0..#expr {
                        #inner_len
                    }
                }
            }
            SupportedType::Defined(_) => quote!(offset += self.#name().total_len();),
        }
    }
}

impl TryFrom<&Type> for SupportedType {
    type Error = syn::Error;

    fn try_from(ty: &Type) -> Result<Self, Self::Error> {
        match ty {
            Type::Array(ref type_array) => Ok(SupportedType::Array(
                Box::new(type_array.elem.as_ref().try_into()?),
                type_array.len.clone(),
            )),
            Type::Path(type_path) => {
                let name = type_path
                    .path
                    .segments
                    .last()
                    .ok_or(syn::Error::new_spanned(type_path, "Invalid borsh type."))?;
                let name_string = name.ident.to_string();
                match name_string.as_str() {
                    "bool" => Ok(SupportedType::Bool),
                    "u8" => Ok(SupportedType::U8),
                    "i8" => Ok(SupportedType::I8),
                    "u16" => Ok(SupportedType::U16),
                    "i16" => Ok(SupportedType::I16),
                    "u32" => Ok(SupportedType::U32),
                    "i32" => Ok(SupportedType::I32),
                    "u64" => Ok(SupportedType::U64),
                    "i64" => Ok(SupportedType::I64),
                    "u128" => Ok(SupportedType::U128),
                    "i128" => Ok(SupportedType::I128),
                    "Pubkey" => Ok(SupportedType::Pubkey),
                    "String" => Ok(SupportedType::String),
                    "Vec" => {
                        let inner_ty = extract_first_generic_type(name, "Vec")?;
                        Ok(SupportedType::Vec(Box::new(inner_ty.try_into()?)))
                    }
                    "Option" => {
                        let inner_ty = extract_first_generic_type(name, "Option")?;
                        Ok(SupportedType::Option(Box::new(inner_ty.try_into()?)))
                    }
                    "HashSet" | "HashMap" => unimplemented!(),
                    _ => Ok(SupportedType::Defined(name_string)),
                }
            }
            _ => unimplemented!(),
        }
    }
}

fn extract_first_generic_type<'a>(
    name: &'a syn::PathSegment,
    type_name: &str,
) -> syn::Result<&'a Type> {
    let PathArguments::AngleBracketed(AngleBracketedGenericArguments { ref args, .. }) =
        name.arguments
    else {
        return Err(syn::Error::new_spanned(
            name,
            format!("Invalid {type_name} type"),
        ));
    };

    let Some(GenericArgument::Type(ty)) = args.first() else {
        return Err(syn::Error::new_spanned(
            name,
            format!("Invalid {type_name} type"),
        ));
    };

    Ok(ty)
}

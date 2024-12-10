use syn::{spanned::Spanned, PathArguments, Type as SynType};

#[derive(PartialEq, Debug)]
pub enum Type {
    Bool,
    Number { signed: bool, size: NumberSize },
    Float { size: FloatSize },
    Pubkey,
    String,
    Vec(Box<Type>),
    Array(Box<Type>, Len),
    Option(Box<Type>),
    Defined(String),
}

#[derive(PartialEq, Debug)]
pub enum Len {
    Number(usize),
    Const(String),
}

#[derive(PartialEq, Debug)]
pub enum NumberSize {
    One,
    Two,
    Four,
    Eight,
    Sixteen,
}

#[derive(PartialEq, Debug)]
pub enum FloatSize {
    Four,
    Eight,
}

impl TryFrom<&SynType> for Type {
    type Error = syn::Error;

    fn try_from(value: &SynType) -> Result<Self, Self::Error> {
        match value {
            SynType::Path(type_path) => {
                let Some(segment) = type_path.path.segments.last() else {
                    return Err(syn::Error::new(type_path.span(), "Invalid segment."));
                };
                let name_ty = segment.ident.to_string();

                match name_ty.as_str() {
                    "bool" => Ok(Type::Bool),
                    "Pubkey" => Ok(Type::Pubkey),
                    "u8" => Ok(Type::Number {
                        signed: false,
                        size: NumberSize::One,
                    }),
                    "u16" => Ok(Type::Number {
                        signed: false,
                        size: NumberSize::Two,
                    }),
                    "u32" => Ok(Type::Number {
                        signed: false,
                        size: NumberSize::Four,
                    }),
                    "u64" => Ok(Type::Number {
                        signed: false,
                        size: NumberSize::Eight,
                    }),
                    "u128" => Ok(Type::Number {
                        signed: false,
                        size: NumberSize::Sixteen,
                    }),
                    "i8" => Ok(Type::Number {
                        signed: true,
                        size: NumberSize::One,
                    }),
                    "i16" => Ok(Type::Number {
                        signed: true,
                        size: NumberSize::Two,
                    }),
                    "i32" => Ok(Type::Number {
                        signed: true,
                        size: NumberSize::Four,
                    }),
                    "i64" => Ok(Type::Number {
                        signed: true,
                        size: NumberSize::Eight,
                    }),
                    "i128" => Ok(Type::Number {
                        signed: true,
                        size: NumberSize::Sixteen,
                    }),
                    "f32" => Ok(Type::Float {
                        size: FloatSize::Four,
                    }),
                    "f64" => Ok(Type::Float {
                        size: FloatSize::Eight,
                    }),
                    "String" | "str" => Ok(Type::String),
                    "Vec" => {
                        let PathArguments::AngleBracketed(generic_args) = &segment.arguments else {
                            return Err(syn::Error::new(segment.span(), "Invalid Vec type."));
                        };

                        let Some(syn::GenericArgument::Type(ty)) = generic_args.args.first() else {
                            return Err(syn::Error::new(segment.span(), "Invalid Vec argument."));
                        };

                        Ok(Type::Vec(Box::new(Type::try_from(ty)?)))
                    }
                    "Option" => todo!(),
                    _ => Ok(Type::Defined(name_ty)),
                }
            }
            SynType::Array(_type_array) => todo!(),
            SynType::Reference(type_ref) => Type::try_from(type_ref.elem.as_ref()),
            _ => Err(syn::Error::new(value.span(), "Invalid type used.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, syn::parse_quote};

    #[test]
    fn parse_bool() {
        let ty: SynType = parse_quote!(bool);

        let parsed_ty = Type::try_from(&ty).unwrap();
        assert_eq!(parsed_ty, Type::Bool);
    }

    macro_rules! test_number {
        ($ty:ty, $signed:literal, $size:ident) => {
            let test_number: SynType = parse_quote!($ty);

            let parsed_ty = Type::try_from(&test_number).unwrap();
            assert_eq!(
                parsed_ty,
                Type::Number {
                    signed: $signed,
                    size: NumberSize::$size
                }
            );
        };
    }

    #[test]
    fn parse_numbers() {
        // Signed
        test_number!(i8, true, One);
        test_number!(i16, true, Two);
        test_number!(i32, true, Four);
        test_number!(i64, true, Eight);
        test_number!(i128, true, Sixteen);

        // Unsigned
        test_number!(u8, false, One);
        test_number!(u16, false, Two);
        test_number!(u32, false, Four);
        test_number!(u64, false, Eight);
        test_number!(u128, false, Sixteen);
    }

    #[test]
    fn parse_string() {
        let ty: SynType = parse_quote!(String);
        let parsed_ty = Type::try_from(&ty).unwrap();
        assert_eq!(parsed_ty, Type::String);

        let ty: SynType = parse_quote!(&'static str);
        let parsed_ty = Type::try_from(&ty).unwrap();
        assert_eq!(parsed_ty, Type::String);
    }

    #[test]
    fn parse_floats() {
        let ty: SynType = parse_quote!(f32);

        let parsed_ty = Type::try_from(&ty).unwrap();
        assert_eq!(
            parsed_ty,
            Type::Float {
                size: FloatSize::Four
            }
        );

        let ty: SynType = parse_quote!(f64);

        let parsed_ty = Type::try_from(&ty).unwrap();
        assert_eq!(
            parsed_ty,
            Type::Float {
                size: FloatSize::Eight
            }
        );
    }

    #[test]
    fn parse_pubkey() {
        let ty: SynType = parse_quote!(Pubkey);

        let parsed_ty = Type::try_from(&ty).unwrap();
        assert_eq!(parsed_ty, Type::Pubkey);

        let ty: SynType = parse_quote!(solana_program::pubkey::Pubkey);

        let parsed_ty = Type::try_from(&ty).unwrap();
        assert_eq!(parsed_ty, Type::Pubkey);
    }
}

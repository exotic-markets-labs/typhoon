use {
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{
        parse::{Parse, ParseStream},
        parse2,
        spanned::Spanned,
        Attribute, Ident, Path, PathSegment, Token,
    },
};

pub enum Argument {
    Value {
        name: Ident,
        ty: Option<PathSegment>,
    },
    Struct {
        name: Ident,
    },
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        if input.is_empty() {
            Ok(Argument::Struct { name })
        } else {
            input.parse::<Token![:]>()?;
            let ty: Path = input.parse()?;
            let path_segment = ty
                .segments
                .first()
                .ok_or_else(|| {
                    syn::Error::new(ty.span(), "Expected at least one path segment for the type")
                })?
                .clone();
            Ok(Argument::Value {
                name,
                ty: Some(path_segment),
            })
        }
    }
}

pub enum Arguments {
    Values(Vec<Argument>),
    Struct(Argument),
}

impl Arguments {
    pub fn get_name(&self, base_name: &Ident) -> Ident {
        match self {
            Arguments::Struct(Argument::Struct { name }) => name.to_owned(),
            Arguments::Values(_) => format_ident!("{}Args", base_name),
            _ => {
                panic!("Can't determine if args are values or a struct")
            }
        }
    }

    pub fn generate_struct(&self, struct_name: &Ident) -> Option<TokenStream> {
        if let Arguments::Values(list) = self {
            let fields = list.iter().map(|arg| {
                if let Argument::Value { name, ty } = arg {
                    let name = &name;
                    let ty = &ty.clone().unwrap().ident;
                    quote! {
                        pub #name: #ty,
                    }
                } else {
                    quote!()
                }
            });

            let generated_struct = quote! {
                #[repr(C)]
                #[derive(Debug, PartialEq, zerocopy::KnownLayout, zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::FromBytes)]
                pub struct #struct_name {
                    #(#fields)*
                }
            };

            Some(generated_struct)
        } else {
            None
        }
    }
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut arguments = Vec::new();
        while !input.is_empty() {
            let arg: Argument = input.parse()?;

            if let Argument::Struct { name } = &arg {
                if !input.is_empty() || !arguments.is_empty() {
                    return Err(syn::Error::new(
                        name.span(),
                        "User defined struct in argument should be used alone",
                    ));
                }

                return Ok(Arguments::Struct(arg));
            }

            arguments.push(arg);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Arguments::Values(arguments))
    }
}

impl TryFrom<&mut Attribute> for Arguments {
    type Error = syn::Error;

    fn try_from(value: &mut Attribute) -> Result<Self, Self::Error> {
        let tokens = value.meta.require_list()?.tokens.clone();
        parse2::<Arguments>(tokens)
    }
}

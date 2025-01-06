use {
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{
        parse::{Parse, ParseStream},
        parse2, Attribute, Ident, Token, Type,
    },
};

pub enum Argument {
    Value { name: Ident, ty: Type },
    Struct { name: Ident },
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        if input.is_empty() {
            Ok(Argument::Struct { name })
        } else {
            input.parse::<Token![:]>()?;

            let ty: Type = input.parse()?;
            Ok(Argument::Value { name, ty })
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
                    quote! {
                        pub #name: #ty,
                    }
                } else {
                    quote!()
                }
            });

            let generated_struct = quote! {
                #[derive(Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
                #[repr(C)]
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

use syn::{
    parse::{Parse, ParseStream},
    parse2, Attribute, Ident, Token, Type,
};

#[derive(Clone)]
pub struct Argument {
    pub name: Ident,
    pub ty: Type,
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;

        Ok(Argument { name, ty })
    }
}

pub enum Arguments {
    Values(Vec<Argument>),
    Struct(Ident),
}
impl Parse for Arguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(Token![:]) {
            let mut arguments = Vec::new();
            while input.peek2(Token![:]) {
                let arg: Argument = input.parse()?;
                arguments.push(arg);

                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }
            }

            Ok(Arguments::Values(arguments))
        } else {
            Ok(Arguments::Struct(input.parse()?))
        }
    }
}

impl TryFrom<&Attribute> for Arguments {
    type Error = syn::Error;

    fn try_from(value: &Attribute) -> Result<Self, Self::Error> {
        let tokens = value.meta.require_list()?.tokens.clone();
        parse2::<Arguments>(tokens)
    }
}

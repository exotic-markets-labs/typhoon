use syn::{parse::Parse, Ident, Token};

#[derive(Clone)]
pub enum ConstraintToken {
    Mint(Ident),
    Authority(Ident),
}

impl Parse for ConstraintToken {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![::]>()?;
        let name = input.parse::<Ident>()?.to_string();
        match name.as_str() {
            "mint" => {
                input.parse::<Token![=]>()?;

                Ok(ConstraintToken::Mint(input.parse()?))
            }
            "authority" => {
                input.parse::<Token![=]>()?;

                Ok(ConstraintToken::Authority(input.parse()?))
            }
            _ => Err(syn::Error::new(
                input.span(),
                "Invalid variant for the token constraint.",
            )),
        }
    }
}

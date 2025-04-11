use syn::{parse::Parse, Expr, Ident, Token};

#[derive(Clone)]
pub enum ConstraintMint {
    Authority(Expr),
    Decimals(Expr),
    FreezeAuthority(Expr),
}

impl Parse for ConstraintMint {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![::]>()?;
        let name = input.parse::<Ident>()?.to_string();
        match name.as_str() {
            "authority" => {
                input.parse::<Token![=]>()?;

                Ok(ConstraintMint::Authority(input.parse()?))
            }
            "decimals" => {
                input.parse::<Token![=]>()?;

                Ok(ConstraintMint::Decimals(input.parse()?))
            }
            "freeze_authority" => {
                input.parse::<Token![=]>()?;
                Ok(ConstraintMint::FreezeAuthority(input.parse()?))
            }
            _ => Err(syn::Error::new(
                input.span(),
                "Invalid variant for the token constraint.",
            )),
        }
    }
}

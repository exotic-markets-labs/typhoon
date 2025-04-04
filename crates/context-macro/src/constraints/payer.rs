use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

#[derive(Clone)]
pub struct ConstraintPayer {
    pub target: Ident,
}

impl Parse for ConstraintPayer {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let target = input.parse()?;

        Ok(ConstraintPayer { target })
    }
}

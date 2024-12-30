use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

#[derive(Clone)]
pub struct ConstraintPayer {
    pub target: Expr,
}

impl Parse for ConstraintPayer {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let target = input.parse()?;

        Ok(ConstraintPayer { target })
    }
}

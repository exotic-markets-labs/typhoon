use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

pub struct ConstraintPayer {
    pub target: Expr,
}

impl Parse for ConstraintPayer {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _punct: Token![=] = input.parse()?;
        let target = input.parse()?;

        Ok(ConstraintPayer { target })
    }
}

use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

#[derive(Clone)]
pub struct ConstraintSpace {
    pub space: Expr,
}

impl Parse for ConstraintSpace {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _punct: Token![=] = input.parse()?;
        let space = input.parse()?;

        Ok(ConstraintSpace { space })
    }
}

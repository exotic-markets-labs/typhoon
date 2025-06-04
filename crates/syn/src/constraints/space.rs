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
        input.parse::<Token![=]>()?;
        let space = input.parse()?;

        Ok(ConstraintSpace { space })
    }
}

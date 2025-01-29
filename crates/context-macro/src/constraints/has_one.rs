use syn::{parse::Parse, Expr, Token};

#[derive(Clone)]
pub struct ConstraintHasOne {
    pub join_target: Expr,
    pub error: Option<Expr>,
}

impl Parse for ConstraintHasOne {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let join_target = input.parse()?;
        let error = if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(ConstraintHasOne { join_target, error })
    }
}

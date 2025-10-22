use {
    crate::utils::ContextExpr,
    syn::{parse::Parse, Expr, Token},
};

#[derive(Clone)]
pub struct ConstraintAssert {
    pub assert: ContextExpr,
    pub error: Option<Expr>,
}

impl Parse for ConstraintAssert {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let assert: ContextExpr = input.parse()?;
        let error = if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(ConstraintAssert { assert, error })
    }
}

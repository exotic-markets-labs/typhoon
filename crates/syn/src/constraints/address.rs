use {
    crate::utils::ContextExpr,
    syn::{parse::Parse, Expr, Token},
};

#[derive(Clone)]
pub struct ConstraintAddress {
    pub check: ContextExpr,
    pub error: Option<Expr>,
}

impl Parse for ConstraintAddress {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let check: ContextExpr = input.parse()?;
        let error = if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(ConstraintAddress { check, error })
    }
}

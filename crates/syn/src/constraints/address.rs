use syn::{parse::Parse, Expr, Token};

use crate::utils::ContextExpr;

#[derive(Clone)]
pub struct ConstraintAddress {
    pub address: ContextExpr,
    pub error: Option<Expr>,
}

impl Parse for ConstraintAddress {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let address: ContextExpr = input.parse()?;
        let error = if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(ConstraintAddress { address, error })
    }
}

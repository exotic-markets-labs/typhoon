use {
    crate::utils::SeedsExpr,
    syn::{
        parse::{Parse, ParseStream},
        Token,
    },
};

#[derive(Clone)]
pub struct ConstraintSeeded(pub Option<SeedsExpr>);

impl Parse for ConstraintSeeded {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;

            let seeds = input.parse()?;
            Ok(ConstraintSeeded(Some(seeds)))
        } else {
            Ok(ConstraintSeeded(None))
        }
    }
}

use {
    crate::utils::ContextExpr,
    syn::{
        parse::{Parse, ParseStream},
        Token,
    },
};

#[derive(Clone)]
pub struct ConstraintBump(pub Option<ContextExpr>);

impl Parse for ConstraintBump {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let bump = input.parse()?;

            Ok(ConstraintBump(Some(bump)))
        } else {
            Ok(ConstraintBump(None))
        }
    }
}

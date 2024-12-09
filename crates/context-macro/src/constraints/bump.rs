use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

pub struct ConstraintBump {
    pub bump: Expr,
}

impl Parse for ConstraintBump {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _punct: Token![=] = input.parse()?;
        let bump = input.parse()?;

        Ok(ConstraintBump { bump })
    }
}

use {
    crate::utils::SeedsExpr,
    syn::{
        parse::{Parse, ParseStream},
        Token,
    },
};

#[derive(Clone)]
pub struct ConstraintSeeds {
    pub seeds: SeedsExpr,
}

impl Parse for ConstraintSeeds {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;

        let seeds = input.parse()?;
        Ok(ConstraintSeeds { seeds })
    }
}

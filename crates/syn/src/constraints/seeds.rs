use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Token,
};

#[derive(Clone)]
pub struct ConstraintSeeds {
    pub seeds: Punctuated<Expr, Token![,]>,
}

impl Parse for ConstraintSeeds {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let content;
        syn::bracketed!(content in input);

        let mut seeds = content.parse_terminated(Expr::parse, Token![,])?;

        if seeds.trailing_punct() {
            seeds.pop_punct();
        }

        Ok(ConstraintSeeds { seeds })
    }
}

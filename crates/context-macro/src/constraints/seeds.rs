use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Token,
};

pub struct ConstraintSeeds {
    pub seeds: Punctuated<Expr, Token![,]>,
}

impl Parse for ConstraintSeeds {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _punct: Token![=] = input.parse()?;
        let content;
        let _bracket_token = syn::bracketed!(content in input);

        let mut seeds = content.parse_terminated(Expr::parse, Token![,])?;

        if seeds.trailing_punct() {
            seeds.pop_punct();
        }

        Ok(ConstraintSeeds { seeds })
    }
}

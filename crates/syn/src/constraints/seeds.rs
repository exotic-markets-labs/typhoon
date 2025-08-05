use {
    crate::utils::SeedsExpr,
    syn::{
        parse::{Parse, ParseStream},
        token::Bracket,
        Expr, Token,
    },
};

#[derive(Clone)]
pub struct ConstraintSeeds {
    pub seeds: SeedsExpr,
}

impl Parse for ConstraintSeeds {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;

        if input.peek(Bracket) {
            let content;
            syn::bracketed!(content in input);

            let mut seeds = content.parse_terminated(Expr::parse, Token![,])?;

            if seeds.trailing_punct() {
                seeds.pop_punct();
            }

            Ok(ConstraintSeeds {
                seeds: SeedsExpr::Punctuated(seeds),
            })
        } else {
            Ok(ConstraintSeeds {
                seeds: SeedsExpr::Single(input.parse()?),
            })
        }
    }
}

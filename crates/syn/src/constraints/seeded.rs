use {
    crate::utils::SeedsExpr,
    syn::{
        parse::{Parse, ParseStream},
        token::Bracket,
        Expr, Token,
    },
};

#[derive(Clone)]
pub struct ConstraintSeeded(pub Option<SeedsExpr>);

impl Parse for ConstraintSeeded {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;

            if input.peek(Bracket) {
                let content;
                syn::bracketed!(content in input);

                let mut seeds = content.parse_terminated(Expr::parse, Token![,])?;

                if seeds.trailing_punct() {
                    seeds.pop_punct();
                }

                Ok(ConstraintSeeded(Some(SeedsExpr::Punctuated(seeds))))
            } else {
                Ok(ConstraintSeeded(Some(SeedsExpr::Single(input.parse()?))))
            }
        } else {
            Ok(ConstraintSeeded(None))
        }
    }
}

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Token,
};

#[derive(Clone)]
pub struct ConstraintSeeded(pub Option<Punctuated<Expr, Token![,]>>);

impl Parse for ConstraintSeeded {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let content;
            syn::bracketed!(content in input);

            let mut seeds = content.parse_terminated(Expr::parse, Token![,])?;

            if seeds.trailing_punct() {
                seeds.pop_punct();
            }

            Ok(ConstraintSeeded(Some(seeds)))
        } else {
            Ok(ConstraintSeeded(None))
        }
    }
}

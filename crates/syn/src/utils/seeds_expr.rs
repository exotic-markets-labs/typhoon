use {
    quote::ToTokens,
    syn::{punctuated::Punctuated, Expr, Token},
};

#[derive(Clone)]
pub enum SeedsExpr {
    Punctuated(Punctuated<Expr, Token![,]>),
    Single(Expr),
}

impl ToTokens for SeedsExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            SeedsExpr::Punctuated(punctuated) => punctuated.to_tokens(tokens),
            SeedsExpr::Single(expr) => expr.to_tokens(tokens),
        }
    }
}

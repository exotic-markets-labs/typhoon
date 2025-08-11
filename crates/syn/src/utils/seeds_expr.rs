use {
    quote::ToTokens,
    syn::{parse::Parse, punctuated::Punctuated, Expr, Token},
};

#[derive(Clone)]
pub enum SeedsExpr {
    Punctuated(Punctuated<Expr, Token![,]>),
    Single(Expr),
}

impl Parse for SeedsExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr: Expr = input.parse()?;

        match expr {
            Expr::Array(array_expr) => {
                let mut elems = array_expr.elems;
                if elems.trailing_punct() {
                    elems.pop_punct();
                }
                Ok(SeedsExpr::Punctuated(elems))
            }
            Expr::Call(_) | Expr::MethodCall(_) => Ok(SeedsExpr::Single(expr)),
            _ => Err(syn::Error::new_spanned(&expr, "Invalid seeds format")),
        }
    }
}

impl ToTokens for SeedsExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            SeedsExpr::Punctuated(punctuated) => punctuated.to_tokens(tokens),
            SeedsExpr::Single(expr) => expr.to_tokens(tokens),
        }
    }
}

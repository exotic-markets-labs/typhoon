use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Token,
};

#[derive(Clone)]
pub struct ConstraintKeys {
    pub keys: Punctuated<Expr, Token![,]>,
}

impl Parse for ConstraintKeys {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let content;
        syn::bracketed!(content in input);

        let mut keys = content.parse_terminated(Expr::parse, Token![,])?;

        if keys.trailing_punct() {
            keys.pop_punct();
        }

        Ok(ConstraintKeys { keys })
    }
}

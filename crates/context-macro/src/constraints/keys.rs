use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Token,
};

pub struct ConstraintKeys {
    pub keys: Punctuated<Expr, Token![,]>,
}

impl Parse for ConstraintKeys {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _punct: Token![=] = input.parse()?;
        let content;
        let _bracket_token = syn::bracketed!(content in input);

        let mut keys = content.parse_terminated(Expr::parse, Token![,])?;

        if keys.trailing_punct() {
            keys.pop_punct();
        }

        Ok(ConstraintKeys { keys })
    }
}

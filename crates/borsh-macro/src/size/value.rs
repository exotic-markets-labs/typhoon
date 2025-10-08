use {
    proc_macro2::TokenStream as TokenStream2,
    quote::{quote, ToTokens},
    syn::{parse::Parse, Ident, LitInt},
};

pub enum Value {
    Lit(LitInt),
    Ident(Ident),
}

impl Parse for Value {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            input.parse().map(Value::Ident)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Value::Lit)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let expanded = match self {
            Value::Lit(lit) => quote!(#lit),
            Value::Ident(ident) => quote!((#ident as usize)),
        };

        expanded.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use {super::*, syn::parse_quote};

    #[test]
    fn const_value() {
        let val: Value = parse_quote!(LEN);
        let token = val.into_token_stream().to_string();

        assert_eq!(token, "(LEN as usize)");
    }

    #[test]
    fn num_value() {
        let val: Value = parse_quote!(123);
        let token = val.into_token_stream().to_string();

        assert_eq!(token, "123");
    }
}

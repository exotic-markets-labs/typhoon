use {
    crate::size::value::Value,
    quote::ToTokens,
    std::ops::{Deref, DerefMut},
    syn::{
        parenthesized, parse::Parse, parse2, punctuated::Punctuated, spanned::Spanned,
        token::Paren, Attribute, Ident, Token,
    },
};

pub struct Arg {
    pub is_raw: bool,
    pub value: Value,
}

impl Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(Paren) {
            let ident: Ident = input.parse()?;

            if ident != "raw_space" {
                return Err(syn::Error::new(
                    input.span(),
                    "The raw arg must be right like 'raw_space(your value)'",
                ));
            }

            let content;
            parenthesized!(content in input);

            Ok(Arg {
                is_raw: true,
                value: content.parse()?,
            })
        } else {
            Ok(Arg {
                is_raw: false,
                value: input.parse()?,
            })
        }
    }
}

#[derive(Default)]
pub struct Args(pub Vec<Arg>);

impl Deref for Args {
    type Target = Vec<Arg>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Args {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<&Attribute> for Args {
    type Error = syn::Error;

    fn try_from(value: &Attribute) -> Result<Self, Self::Error> {
        if value.path().is_ident("max_len") {
            let ty_lens: Punctuated<Arg, Token![,]> = value
                .meta
                .require_list()?
                .parse_args_with(Punctuated::parse_terminated)?;

            Ok(Args(ty_lens.into_iter().rev().collect()))
        } else if value.path().is_ident("raw_space") {
            let value = parse2(value.meta.require_list()?.into_token_stream())?;

            Ok(Args(vec![value]))
        } else {
            Err(syn::Error::new(value.span(), "Invalid attribute."))
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, quote::ToTokens, syn::parse_quote};

    fn stringify_value(v: &Value) -> String {
        v.into_token_stream().to_string()
    }

    #[test]
    fn basic_max_len_attr() {
        let attr: Attribute = parse_quote!(#[max_len(10)]);
        let args = Args::try_from(&attr).unwrap();

        let arg = &args[0];

        assert!(!arg.is_raw);
        assert_eq!(stringify_value(&arg.value), "10");
    }

    #[test]
    fn complex_max_len_attr() {
        let attr: Attribute = parse_quote!(#[max_len(10, raw_space(40), LEN, raw_space(LEN))]);
        let args = Args::try_from(&attr).unwrap();

        let arg = &args[0];
        assert!(arg.is_raw);
        assert_eq!(stringify_value(&arg.value), "(LEN as usize)");

        let arg = &args[1];
        assert!(!arg.is_raw);
        assert_eq!(stringify_value(&arg.value), "(LEN as usize)");

        let arg = &args[2];
        assert!(arg.is_raw);
        assert_eq!(stringify_value(&arg.value), "40");

        let arg = &args[3];
        assert!(!arg.is_raw);
        assert_eq!(stringify_value(&arg.value), "10");
    }

    #[test]
    #[should_panic]
    fn wrong_max_len_attr() {
        let attr: Attribute = parse_quote!(#[max_len(raw_space(raw_space(10)))]);
        Args::try_from(&attr).unwrap();
    }

    #[test]
    fn lit_raw_space_attr() {
        let attr: Attribute = parse_quote!(#[raw_space(10)]);
        let args = Args::try_from(&attr).unwrap();

        let arg = &args[0];

        assert!(arg.is_raw);
        assert_eq!(stringify_value(&arg.value), "10");
    }

    #[test]
    fn const_raw_space_attr() {
        let attr: Attribute = parse_quote!(#[raw_space(LEN)]);
        let args = Args::try_from(&attr).unwrap();

        let arg = &args[0];

        assert!(arg.is_raw);
        assert_eq!(stringify_value(&arg.value), "(LEN as usize)");
    }

    #[test]
    #[should_panic]
    fn wrong_raw_space_attr() {
        let attr: Attribute = parse_quote!(#[raw_space("Ok")]);
        Args::try_from(&attr).unwrap();
    }
}

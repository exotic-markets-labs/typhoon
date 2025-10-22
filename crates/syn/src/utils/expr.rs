use {
    quote::{format_ident, quote, ToTokens},
    syn::{
        fold::{fold_expr, Fold},
        parse::Parse,
        parse_quote, Expr, Ident,
    },
};

#[derive(Clone)]
pub struct ContextExpr {
    pub names: Vec<Ident>,
    expr: Expr,
}

impl Parse for ContextExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = Expr::parse(input)?;

        Ok(ContextExpr::from(expr.clone()))
    }
}

impl ToTokens for ContextExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expr = &self.expr;
        quote!(#expr).to_tokens(tokens)
    }
}

impl From<Expr> for ContextExpr {
    fn from(value: Expr) -> Self {
        let mut names = Names::default();
        let expr = names.fold_expr(value);
        ContextExpr {
            names: names.0,
            expr,
        }
    }
}

#[derive(Default)]
pub struct Names(Vec<Ident>);

impl Fold for Names {
    fn fold_expr(&mut self, i: syn::Expr) -> syn::Expr {
        let Expr::Try(ref try_expr) = i else {
            return fold_expr(self, i);
        };

        let Expr::MethodCall(ref method_call) = try_expr.expr.as_ref() else {
            return fold_expr(self, i);
        };

        if method_call.method != "data" {
            return fold_expr(self, i);
        }

        let Expr::Path(name) = method_call.receiver.as_ref() else {
            return fold_expr(self, i);
        };

        let Some(name_ident) = name.path.get_ident().cloned() else {
            return fold_expr(self, i);
        };

        let ident = format_ident!("{}_state", name_ident);

        self.0.push(name_ident);

        parse_quote!(#ident)
    }
}

#[cfg(test)]
mod from_expr_tests {
    use {
        super::*,
        quote::{quote, ToTokens},
        syn::parse_quote,
    };

    #[test]
    fn test_method_call_with_try() {
        // Test for pattern: counter.data()?.bump()
        let expr: Expr = parse_quote!(counter.data()?.bump());
        let context_expr = ContextExpr::from(expr);

        let inner_expr = context_expr.to_token_stream();
        let expected_expr = quote!(counter_state.bump());
        assert_eq!(expected_expr.to_string(), inner_expr.to_string());
        assert_eq!(context_expr.names.len(), 1);
    }

    #[test]
    fn test_field_access_with_try() {
        // Test for pattern: counter.data()?.bump
        let expr: Expr = parse_quote!(counter.data()?.bump);
        let context_expr = ContextExpr::from(expr);

        let inner_expr = context_expr.to_token_stream().to_string();
        let expected_expr = quote!(counter_state.bump);
        assert_eq!(expected_expr.to_string(), inner_expr.to_string());
        assert_eq!(context_expr.names.len(), 1);
    }

    #[test]
    fn test_other_expr() {
        let expr: Expr = parse_quote!(counter.random()?.bump);
        let context_expr = ContextExpr::from(expr);

        assert!(context_expr.names.is_empty());

        let inner_expr = context_expr.to_token_stream().to_string();
        let expected_expr = quote!(counter.random()?.bump);
        assert_eq!(expected_expr.to_string(), inner_expr.to_string());
    }
}

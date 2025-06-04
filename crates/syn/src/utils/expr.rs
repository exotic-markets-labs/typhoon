use {
    quote::{format_ident, quote, ToTokens},
    syn::{parse::Parse, Expr, Ident},
};

#[derive(Clone)]
pub struct ContextExpr {
    name: Option<Ident>,
    expr: Expr,
}

impl ContextExpr {
    pub fn name(&self) -> Option<&Ident> {
        self.name.as_ref()
    }
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
        if let Some(name) = &self.name {
            let state_name = format_ident!("{name}_state");
            quote!(#state_name #expr).to_tokens(tokens);
        } else {
            quote!(#expr).to_tokens(tokens);
        }
    }
}

impl From<Expr> for ContextExpr {
    fn from(value: Expr) -> Self {
        Self::from_expr(&value).unwrap_or(ContextExpr {
            name: None,
            expr: value,
        })
    }
}

impl ContextExpr {
    fn from_expr(expr: &Expr) -> Result<ContextExpr, syn::Error> {
        match expr {
            Expr::MethodCall(method_call) => {
                let name = Self::extract_name(method_call.receiver.as_ref())?;
                let expr = Self::create_method_call_expr(method_call);
                Ok(ContextExpr {
                    name: Some(name),
                    expr,
                })
            }
            Expr::Field(field_expr) => {
                let name = Self::extract_name(field_expr.base.as_ref())?;
                let expr = Self::create_field_expr(field_expr);
                Ok(ContextExpr {
                    name: Some(name),
                    expr,
                })
            }
            _ => Err(syn::Error::new_spanned(expr, "Unsupported expression type")),
        }
    }

    fn extract_name(expr: &Expr) -> Result<Ident, syn::Error> {
        let try_expr = match expr {
            Expr::Try(ref try_expr) => try_expr,
            _ => return Err(syn::Error::new_spanned(expr, "Expected try expression")),
        };

        let inner_method_call = match try_expr.expr.as_ref() {
            Expr::MethodCall(ref inner) => inner,
            _ => {
                return Err(syn::Error::new_spanned(
                    &try_expr.expr,
                    "Expected method call after try operator",
                ))
            }
        };

        if inner_method_call.method != syn::Ident::new("data", inner_method_call.method.span()) {
            return Err(syn::Error::new_spanned(
                &inner_method_call.method,
                "Expected 'data' method call",
            ));
        }

        Self::extract_name_from_receiver(&inner_method_call.receiver)
    }

    fn extract_name_from_receiver(receiver: &Expr) -> Result<Ident, syn::Error> {
        match receiver {
            Expr::Path(path_expr) => Ok(path_expr.path.segments.first().unwrap().ident.clone()),
            _ => Err(syn::Error::new_spanned(
                receiver,
                "Expected path expression for method receiver",
            )),
        }
    }

    fn create_method_call_expr(method_call: &syn::ExprMethodCall) -> Expr {
        Expr::MethodCall(syn::ExprMethodCall {
            attrs: method_call.attrs.clone(),
            receiver: Box::new(Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: syn::punctuated::Punctuated::new(),
                },
            })),
            dot_token: method_call.dot_token,
            method: method_call.method.clone(),
            turbofish: method_call.turbofish.clone(),
            paren_token: method_call.paren_token,
            args: method_call.args.clone(),
        })
    }

    fn create_field_expr(field_expr: &syn::ExprField) -> Expr {
        Expr::Field(syn::ExprField {
            attrs: field_expr.attrs.clone(),
            base: Box::new(Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: syn::punctuated::Punctuated::new(),
                },
            })),
            dot_token: field_expr.dot_token,
            member: field_expr.member.clone(),
        })
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
    }

    #[test]
    fn test_field_access_with_try() {
        // Test for pattern: counter.data()?.bump
        let expr: Expr = parse_quote!(counter.data()?.bump);
        let context_expr = ContextExpr::from(expr);

        let inner_expr = context_expr.to_token_stream().to_string();
        let expected_expr = quote!(counter_state.bump);
        assert_eq!(expected_expr.to_string(), inner_expr.to_string());
    }

    #[test]
    fn test_other_expr() {
        let expr: Expr = parse_quote!(counter.random()?.bump);
        let context_expr = ContextExpr::from(expr);

        assert_eq!(context_expr.name, None);

        let inner_expr = context_expr.to_token_stream().to_string();
        let expected_expr = quote!(counter.random()?.bump);
        assert_eq!(expected_expr.to_string(), inner_expr.to_string());
    }
}

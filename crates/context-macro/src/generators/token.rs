use {
    crate::{
        constraints::{ConstraintInit, ConstraintInitIfNeeded, ConstraintToken},
        visitor::ContextVisitor,
        StagedGenerator,
    },
    quote::{format_ident, quote},
    syn::{Expr, Ident},
};

#[derive(Default)]
struct TokenChecks {
    mint: Option<Ident>,
    authority: Option<Expr>,
    has_init: bool,
}

impl ContextVisitor for TokenChecks {
    fn visit_token(&mut self, constraint: &ConstraintToken) -> Result<(), syn::Error> {
        match constraint {
            ConstraintToken::Mint(ident) => self.mint = Some(ident.clone()),
            ConstraintToken::Authority(expr) => self.authority = Some(expr.clone()),
        }
        Ok(())
    }

    fn visit_init(&mut self, _constraint: &ConstraintInit) -> Result<(), syn::Error> {
        self.has_init = true;
        Ok(())
    }

    fn visit_init_if_needed(
        &mut self,
        _constraint: &ConstraintInitIfNeeded,
    ) -> Result<(), syn::Error> {
        self.has_init = true;
        Ok(())
    }
}

pub struct TokenAccountGenerator;

// impl<'a> TokenAccountGenerator<'a> {
//     pub fn new(context: &'a Context) -> Self {
//         Self { context }
//     }
// }

impl StagedGenerator for TokenAccountGenerator {
    fn append(&mut self, context: &mut crate::GenerationContext) -> Result<(), syn::Error> {
        for account in &context.input.accounts {
            let mut checks = TokenChecks::default();
            checks.visit_account(account)?;

            if (checks.authority.is_some() || checks.mint.is_some()) && !checks.has_init {
                let mut check_token = Vec::with_capacity(2);
                let name = &account.name;
                let var_name = format_ident!("{}_state", name);

                if let Some(authority) = checks.authority {
                    check_token.push(quote! {
                        if &#var_name.authority() != #authority.key() {
                            return Err(Error::TokenConstraintViolated.into());
                        }
                    });
                }

                if let Some(mint) = checks.mint {
                    check_token.push(quote! {
                        if &#var_name.mint() != #mint.key() {
                            return Err(Error::TokenConstraintViolated.into());
                        }
                    });
                }

                context.generated_results.inside.extend(quote! {
                    {
                        let #var_name = #name.data()?;
                        #(#check_token)*
                    }
                });
            }
        }
        Ok(())
    }
}

use {
    super::GeneratorResult,
    crate::{context::Context, visitor::ContextVisitor, StagedGenerator},
    quote::{format_ident, quote},
    syn::{Expr, Ident},
    typhoon_syn::constraints::{ConstraintInit, ConstraintInitIfNeeded, ConstraintToken},
};

#[derive(Default)]
struct TokenChecks {
    mint: Option<Ident>,
    owner: Option<Expr>,
    has_init: bool,
}

impl ContextVisitor for TokenChecks {
    fn visit_token(&mut self, constraint: &ConstraintToken) -> Result<(), syn::Error> {
        match constraint {
            ConstraintToken::Mint(ident) => self.mint = Some(ident.clone()),
            ConstraintToken::Owner(expr) => self.owner = Some(expr.clone()),
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

pub struct TokenAccountGenerator<'a>(&'a Context);

impl<'a> TokenAccountGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self(context)
    }
}

impl StagedGenerator for TokenAccountGenerator<'_> {
    fn append(&mut self, result: &mut GeneratorResult) -> Result<(), syn::Error> {
        for account in &self.0.accounts {
            let mut checks = TokenChecks::default();
            checks.visit_account(account)?;

            if (checks.owner.is_some() || checks.mint.is_some()) && !checks.has_init {
                let mut check_token = Vec::with_capacity(2);
                let name = &account.name;
                let var_name = format_ident!("{}_state", name);

                if let Some(owner) = checks.owner {
                    check_token.push(quote! {
                        if #var_name.owner() != #owner.key() {
                            return Err(ErrorCode::TokenConstraintViolated.into());
                        }
                    });
                }

                if let Some(mint) = checks.mint {
                    check_token.push(quote! {
                        if #var_name.mint() != #mint.key() {
                            return Err(ErrorCode::TokenConstraintViolated.into());
                        }
                    });
                }

                result.inside.extend(check_token);
            }
        }
        Ok(())
    }
}

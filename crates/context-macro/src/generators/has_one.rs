use {
    super::GeneratorResult,
    crate::{
        constraints::ConstraintHasOne, context::Context, visitor::ContextVisitor, StagedGenerator,
    },
    quote::{format_ident, quote},
    syn::{parse_quote, Expr, Ident},
};

pub struct HasOneGenerator<'a> {
    context: &'a Context,
    targets: Vec<(Ident, Option<Expr>)>,
}

impl<'a> HasOneGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            targets: Vec::new(),
            context,
        }
    }
}

impl ContextVisitor for HasOneGenerator<'_> {
    fn visit_has_one(&mut self, constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        self.targets
            .push((constraint.join_target.clone(), constraint.error.clone()));
        Ok(())
    }
}

impl StagedGenerator for HasOneGenerator<'_> {
    fn append(&mut self, result: &mut GeneratorResult) -> Result<(), syn::Error> {
        for account in &self.context.accounts {
            self.visit_account(account)?;
            if self.targets.is_empty() {
                continue;
            }

            let name = &account.name;
            let var_name = format_ident!("{}_state", name);
            let basic_error: Expr = parse_quote!(ErrorCode::HasOneConstraint);

            let targets = self.targets.iter().map(|(target, error)| {
                let target = &target;
                let error = error.as_ref().unwrap_or(&basic_error);

                quote! {
                    if &#var_name.#target != #target.key() {
                        return Err(#error.into());
                    }
                }
            });

            result.inside.extend(targets);

            self.targets.clear();
        }

        Ok(())
    }
}

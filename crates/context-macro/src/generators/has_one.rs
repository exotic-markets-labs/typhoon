use {
    crate::{
        constraints::ConstraintHasOne, visitor::ContextVisitor, GenerationContext, StagedGenerator,
    },
    quote::{format_ident, quote},
    syn::{parse_quote, Expr, Ident},
};

pub struct HasOneGenerator {
    targets: Vec<(Ident, Option<Expr>)>,
}

impl HasOneGenerator {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
        }
    }
}

impl ContextVisitor for HasOneGenerator {
    fn visit_has_one(&mut self, constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        self.targets
            .push((constraint.join_target.clone(), constraint.error.clone()));
        Ok(())
    }
}

impl StagedGenerator for HasOneGenerator {
    fn append(&mut self, context: &mut GenerationContext) -> Result<(), syn::Error> {
        for account in &context.input.accounts {
            self.visit_account(account)?;
            if self.targets.is_empty() {
                continue;
            }

            let name = &account.name;
            let var_name = format_ident!("{}_state", name);
            let basic_error: Expr = parse_quote!(Error::HasOneConstraint);

            let targets = self.targets.iter().map(|(target, error)| {
                let target = &target;
                let error = error.as_ref().unwrap_or(&basic_error);

                quote! {
                    if &#var_name.#target != #target.key() {
                        return Err(#error.into());
                    }
                }
            });

            context.generated_results.inside.extend(quote! {
                {
                    let #var_name = #name.data()?;
                    #(#targets)*
                }
            });

            self.targets.clear();
        }

        Ok(())
    }
}

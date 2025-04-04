use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{constraints::ConstraintHasOne, visitor::ConstraintVisitor},
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote},
    syn::{parse_quote, Expr, Ident},
};

pub struct HasOneGenerator {
    name: Option<Ident>,
    targets: Vec<(Ident, Option<Expr>)>,
    result: TokenStream,
}

impl HasOneGenerator {
    pub fn new() -> Self {
        Self {
            name: None,
            targets: Vec::new(),
            result: TokenStream::new(),
        }
    }

    fn extend_result(&mut self) -> Result<(), syn::Error> {
        if self.targets.is_empty() {
            return Ok(());
        }

        let name = self
            .name
            .as_ref()
            .ok_or(syn::Error::new(Span::call_site(), "No account provided"))?;
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

        self.result.extend(quote! {
            {
                let #var_name = #name.data()?;
                #(#targets)*
            }
        });

        Ok(())
    }
}

impl ConstraintGenerator for HasOneGenerator {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        Ok(GeneratorResult {
            after_init: self.result.clone(),
            ..Default::default()
        })
    }
}

impl ConstraintVisitor for HasOneGenerator {
    fn visit_account(&mut self, account: &crate::accounts::Account) -> Result<(), syn::Error> {
        self.name = Some(account.name.clone());
        self.targets = Vec::new();
        self.visit_constraints(&account.constraints)?;
        self.extend_result()
    }

    fn visit_has_one(&mut self, constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        self.targets
            .push((constraint.join_target.clone(), constraint.error.clone()));
        Ok(())
    }
}

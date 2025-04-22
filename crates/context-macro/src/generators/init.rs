use {
    super::tokens_gen::InitTokenGenerator,
    crate::{constraints::Constraint, visitor::ContextVisitor, GenerationContext, StagedGenerator},
    quote::quote,
};

pub struct InitializationGenerator;

impl InitializationGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl StagedGenerator for InitializationGenerator {
    fn append(&mut self, context: &mut GenerationContext) -> Result<(), syn::Error> {
        let accounts = context.input.accounts.iter().filter(|acc| {
            acc.constraints
                .0
                .iter()
                .any(|c| matches!(c, Constraint::Init(_)))
        });

        for account in accounts {
            let name = &account.name;
            let account_ty = &account.ty;

            let mut generator = InitTokenGenerator::new(account);
            generator.visit_account(account)?;
            let token = generator.generate()?;
            context.generated_results.inside.extend(quote! {
                let #name: #account_ty = {
                    #token
                };
            });
        }

        Ok(())
    }
}

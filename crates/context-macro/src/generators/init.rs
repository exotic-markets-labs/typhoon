use {
    super::tokens_gen::InitTokenGenerator,
    crate::{
        accounts::Account, constraints::Constraint, visitor::ContextVisitor, GenerationContext,
        StagedGenerator,
    },
    proc_macro2::Span,
    quote::quote,
};

#[derive(Default)]
pub struct InitializationGenerator {
    need_check_system: bool,
    need_check_token: bool,
    need_check_ata: bool,
}

impl InitializationGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    fn check_prerequisite(&self, context: &[Account], program: &str) -> Result<(), syn::Error> {
        let has_system_program = context
            .iter()
            .any(|acc| acc.ty.ident == "Program" && acc.inner_ty == program);

        if !has_system_program {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "Using `init` requires including the `Program<{}>` account",
                    program
                ),
            ));
        }

        Ok(())
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
            self.need_check_system = true;

            if account.inner_ty == "Mint" || account.inner_ty == "TokenAccount" {
                self.need_check_token = true;
            }

            if account
                .constraints
                .0
                .iter()
                .any(|c| matches!(c, Constraint::AssociatedToken(_)))
            {
                self.need_check_ata = true;
            }

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

        if self.need_check_system {
            self.check_prerequisite(&context.input.accounts, "System")?;

            if self.need_check_token {
                self.check_prerequisite(&context.input.accounts, "TokenProgram")?;
            }

            if self.need_check_ata {
                self.check_prerequisite(&context.input.accounts, "AtaTokenProgram")?;
            }
        }

        Ok(())
    }
}

use {
    super::tokens_gen::{BumpTokenGenerator, InitTokenGenerator},
    crate::{
        constraints::{ConstraintBump, ConstraintInit, ConstraintInitIfNeeded},
        context::Context,
        visitor::ContextVisitor,
        StagedGenerator,
    },
    quote::quote,
};

#[derive(Default)]
struct Checks {
    has_bump: bool,
    has_init: bool,
    has_init_if_needed: bool,
}

impl Checks {
    pub fn new() -> Self {
        Checks::default()
    }
}

impl ContextVisitor for Checks {
    fn visit_bump(&mut self, _constraint: &ConstraintBump) -> Result<(), syn::Error> {
        self.has_bump = true;
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
        self.has_init_if_needed = true;
        Ok(())
    }
}

pub struct InitGenerator<'a>(&'a Context);

impl<'a> InitGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self(context)
    }
}

impl StagedGenerator for InitGenerator<'_> {
    fn append(&mut self, result: &mut super::GeneratorResult) -> Result<(), syn::Error> {
        for account in &self.0.accounts {
            let mut checks = Checks::new();
            checks.visit_account(account)?;

            if checks.has_init_if_needed {
                continue;
            }

            if checks.has_bump {
                let mut pda_generator = BumpTokenGenerator::new(account);
                pda_generator.visit_account(account)?;

                let (pda, _, check) = pda_generator.generate()?;

                result.inside.extend(quote! {
                    #pda
                    #check
                });
            }

            if checks.has_init {
                let name = &account.name;
                let account_ty = &account.ty;
                let mut init_gen = InitTokenGenerator::new(account);
                init_gen.visit_account(account)?;
                let init_token = init_gen.generate()?;

                result.inside.extend(quote! {
                    let #name: #account_ty = {
                        #init_token
                    };
                });
            }
        }

        Ok(())
    }
}

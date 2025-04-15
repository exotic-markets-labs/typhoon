use crate::{accounts::Account, arguments::Arguments, constraints::*, context::Context};

pub trait ContextVisitor {
    fn visit_context(&mut self, context: &Context) -> Result<(), syn::Error> {
        self.visit_accounts(&context.accounts)?;

        if let Some(args) = &context.args {
            self.visit_arguments(args)?;
        }

        Ok(())
    }

    fn visit_arguments(&mut self, _arguments: &Arguments) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_accounts(&mut self, accounts: &Vec<Account>) -> Result<(), syn::Error> {
        for account in accounts {
            self.visit_account(account)?;
        }

        Ok(())
    }

    fn visit_account(&mut self, account: &Account) -> Result<(), syn::Error> {
        self.visit_constraints(&account.constraints)
    }

    fn visit_constraints(&mut self, constraints: &Constraints) -> Result<(), syn::Error> {
        for constraint in &constraints.0 {
            self.visit_constraint(constraint)?;
        }

        Ok(())
    }

    fn visit_constraint(&mut self, constraint: &Constraint) -> Result<(), syn::Error> {
        match constraint {
            Constraint::Init(constraint_init) => self.visit_init(constraint_init),
            Constraint::Payer(constraint_payer) => self.visit_payer(constraint_payer),
            Constraint::Space(constraint_space) => self.visit_space(constraint_space),
            Constraint::Seeded(constraint_seeded) => self.visit_seeded(constraint_seeded),
            Constraint::Seeds(constraint_seeds) => self.visit_seeds(constraint_seeds),
            Constraint::Bump(constraint_bump) => self.visit_bump(constraint_bump),
            Constraint::HasOne(constraint_has_one) => self.visit_has_one(constraint_has_one),
            Constraint::Program(constraint_program) => self.visit_program(constraint_program),
            Constraint::Token(constraint_token) => self.visit_token(constraint_token),
            Constraint::Mint(constraint_mint) => self.visit_mint(constraint_mint),
            Constraint::AssociatedToken(constraint_associated_token) => {
                self.visit_associated_token(constraint_associated_token)
            }
            Constraint::InitIfNeeded(contraint_init_if_needed) => {
                self.visit_init_if_needed(contraint_init_if_needed)
            }
        }
    }

    fn visit_init(&mut self, _constraint: &ConstraintInit) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_payer(&mut self, _constraint: &ConstraintPayer) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_space(&mut self, _constraint: &ConstraintSpace) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_seeded(&mut self, _constraint: &ConstraintSeeded) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_seeds(&mut self, _constraint: &ConstraintSeeds) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_bump(&mut self, _constraint: &ConstraintBump) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_has_one(&mut self, _constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_program(&mut self, _constraint: &ConstraintProgram) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_token(&mut self, _constraint: &ConstraintToken) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_mint(&mut self, _constraint: &ConstraintMint) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_associated_token(
        &mut self,
        _constraint: &ConstraintAssociatedToken,
    ) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_init_if_needed(
        &mut self,
        _constraint: &ConstraintInitIfNeeded,
    ) -> Result<(), syn::Error> {
        Ok(())
    }
}

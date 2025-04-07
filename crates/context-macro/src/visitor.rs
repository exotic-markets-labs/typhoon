use crate::{
    accounts::{Account, Accounts},
    constraints::{
        Constraint, ConstraintBump, ConstraintHasOne, ConstraintInit, ConstraintPayer,
        ConstraintSeeded, ConstraintSeeds, ConstraintSpace, Constraints,
    },
};

pub trait ConstraintVisitor {
    fn visit_accounts(&mut self, accounts: &Accounts) -> Result<(), syn::Error> {
        for account in &accounts.0 {
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
}

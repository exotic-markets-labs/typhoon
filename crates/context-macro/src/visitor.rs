use typhoon_syn::{constraints::*, InstructionAccount};

pub trait ContextVisitor {
    fn visit_account(&mut self, account: &InstructionAccount) -> Result<(), syn::Error> {
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
            Constraint::Init(constraint) => self.visit_init(constraint),
            Constraint::Payer(constraint) => self.visit_payer(constraint),
            Constraint::Space(constraint) => self.visit_space(constraint),
            Constraint::Seeded(constraint) => self.visit_seeded(constraint),
            Constraint::Seeds(constraint) => self.visit_seeds(constraint),
            Constraint::Bump(constraint) => self.visit_bump(constraint),
            Constraint::HasOne(constraint) => self.visit_has_one(constraint),
            Constraint::Program(constraint) => self.visit_program(constraint),
            Constraint::Token(constraint) => self.visit_token(constraint),
            Constraint::Mint(constraint) => self.visit_mint(constraint),
            Constraint::AssociatedToken(constraint) => self.visit_associated_token(constraint),
            Constraint::InitIfNeeded(constraint) => self.visit_init_if_needed(constraint),
            Constraint::Assert(constraint) => self.visit_assert(constraint),
            Constraint::Address(constraint) => self.visit_address(constraint),
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

    fn visit_assert(&mut self, _constraint: &ConstraintAssert) -> Result<(), syn::Error> {
        Ok(())
    }

    fn visit_address(&mut self, _constraint: &ConstraintAddress) -> Result<(), syn::Error> {
        Ok(())
    }
}

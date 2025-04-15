use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{
        accounts::Account,
        constraints::{ConstraintAssociatedToken, ConstraintInit},
        visitor::ContextVisitor,
    },
    quote::quote,
};

#[derive(Default)]
pub struct RentGenerator {
    init_counter: i8,
    need_rent: bool,
}

impl RentGenerator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConstraintGenerator for RentGenerator {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        if self.need_rent {
            Ok(GeneratorResult {
                at_init: quote! {
                    let rent = <Rent as Sysvar>::get()?;
                },
                ..Default::default()
            })
        } else {
            Ok(GeneratorResult::default())
        }
    }
}

impl ContextVisitor for RentGenerator {
    fn visit_account(&mut self, account: &Account) -> Result<(), syn::Error> {
        if !self.need_rent {
            self.init_counter = 0;
            self.visit_constraints(&account.constraints)?;
            if self.init_counter > 0 {
                self.need_rent = true;
            }
        }
        Ok(())
    }

    fn visit_init(&mut self, _contraint: &ConstraintInit) -> Result<(), syn::Error> {
        self.init_counter += 1;
        Ok(())
    }

    fn visit_associated_token(
        &mut self,
        _constraint: &ConstraintAssociatedToken,
    ) -> Result<(), syn::Error> {
        self.init_counter -= 1;
        Ok(())
    }
}

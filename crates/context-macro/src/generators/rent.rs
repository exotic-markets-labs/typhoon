use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{constraints::ConstraintInit, visitor::ContextVisitor},
    quote::quote,
};

pub struct RentGenerator(bool);

impl RentGenerator {
    pub fn new() -> Self {
        RentGenerator(false)
    }
}

impl ConstraintGenerator for RentGenerator {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        if self.0 {
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
    fn visit_init(&mut self, _contraint: &ConstraintInit) -> Result<(), syn::Error> {
        self.0 = true; // TODO disable when creating ATA
        Ok(())
    }
}

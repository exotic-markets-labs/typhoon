use {
    crate::{constraints::Constraint, GenerationContext, StagedGenerator},
    quote::quote,
};

pub struct RentGenerator;

impl RentGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl StagedGenerator for RentGenerator {
    fn append(&mut self, context: &mut GenerationContext) -> Result<(), syn::Error> {
        if context.input.accounts.iter().any(|acc| {
            acc.constraints
                .0
                .iter()
                .any(|c| matches!(c, Constraint::Init(_) | Constraint::InitIfNeeded(_)))
        }) {
            context
                .generated_results
                .inside
                .extend(quote!(let rent = <Rent as Sysvar>::get()?;));
        }

        Ok(())
    }
}

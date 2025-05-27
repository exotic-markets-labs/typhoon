use {
    super::GeneratorResult,
    crate::{constraints::Constraint, context::Context, StagedGenerator},
    quote::quote,
};

pub struct RentGenerator<'a>(&'a Context);

impl<'a> RentGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self(context)
    }
}

impl StagedGenerator for RentGenerator<'_> {
    fn append(&mut self, result: &mut GeneratorResult) -> Result<(), syn::Error> {
        if self.0.accounts.iter().any(|acc| {
            acc.constraints
                .0
                .iter()
                .any(|c| matches!(c, Constraint::Init(_) | Constraint::InitIfNeeded(_)))
        }) {
            result
                .inside
                .extend(quote!(let rent = <Rent as Sysvar>::get()?;));
        }

        Ok(())
    }
}

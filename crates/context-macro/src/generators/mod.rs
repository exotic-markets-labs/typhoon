mod arguments;
mod assign;
mod bumps;
mod has_one;
mod init;
mod rent;

use {
    crate::{context::Context, visitor::ContextVisitor},
    proc_macro2::TokenStream,
    syn::Field,
};
pub use {arguments::*, assign::*, bumps::*, has_one::*, init::*, rent::*};

#[derive(Default, Clone)]
pub struct GeneratorResult {
    pub global_outside: TokenStream,
    pub at_init: TokenStream,
    pub after_init: TokenStream,
    pub new_fields: Vec<Field>,
}

pub enum ConstraintGenerators {
    HasOne(HasOneGenerator),
    Init(InitializationGenerator),
    Rent(RentGenerator),
    Args(ArgumentsGenerator),
    Assign(AssignGenerator),
    Bumps(Box<BumpsGenerator>),
}

impl ConstraintGenerator for ConstraintGenerators {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        match self {
            ConstraintGenerators::Bumps(generator) => generator.generate(),
            ConstraintGenerators::HasOne(generator) => generator.generate(),
            ConstraintGenerators::Init(generator) => generator.generate(),
            ConstraintGenerators::Rent(generator) => generator.generate(),
            ConstraintGenerators::Args(generator) => generator.generate(),
            ConstraintGenerators::Assign(generator) => generator.generate(),
        }
    }
}

impl ContextVisitor for ConstraintGenerators {
    fn visit_context(&mut self, context: &Context) -> Result<(), syn::Error> {
        match self {
            ConstraintGenerators::Bumps(generator) => generator.visit_context(context),
            ConstraintGenerators::HasOne(generator) => generator.visit_context(context),
            ConstraintGenerators::Init(generator) => generator.visit_context(context),
            ConstraintGenerators::Rent(generator) => generator.visit_context(context),
            ConstraintGenerators::Args(generator) => generator.visit_context(context),
            ConstraintGenerators::Assign(generator) => generator.visit_context(context),
        }
    }
}

pub trait ConstraintGenerator: ContextVisitor + Sized {
    fn generate(&self) -> Result<GeneratorResult, syn::Error>;
}

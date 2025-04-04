use proc_macro2::TokenStream;

mod bumps;
mod has_one;
mod init;
mod rent;

use {
    crate::{accounts::Accounts, visitor::ConstraintVisitor},
    syn::Field,
};
pub use {bumps::*, has_one::*, init::*, rent::*};

#[derive(Default, Clone)]
pub struct GeneratorResult {
    pub global_outside: TokenStream,
    pub at_init: TokenStream,
    pub after_init: TokenStream,
    pub new_fields: Vec<Field>,
}

pub enum ConstraintGenerators {
    Bumps(BumpsGenerator),
    HasOne(HasOneGenerator),
    Init(InitializationGenerator),
    Rent(RentGenerator),
}

impl ConstraintGenerator for ConstraintGenerators {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        match self {
            ConstraintGenerators::Bumps(generator) => generator.generate(),
            ConstraintGenerators::HasOne(generator) => generator.generate(),
            ConstraintGenerators::Init(generator) => generator.generate(),
            ConstraintGenerators::Rent(generator) => generator.generate(),
        }
    }
}

impl ConstraintVisitor for ConstraintGenerators {
    fn visit_accounts(&mut self, accounts: &Accounts) -> Result<(), syn::Error> {
        match self {
            ConstraintGenerators::Bumps(generator) => generator.visit_accounts(accounts),
            ConstraintGenerators::HasOne(generator) => generator.visit_accounts(accounts),
            ConstraintGenerators::Init(generator) => generator.visit_accounts(accounts),
            ConstraintGenerators::Rent(generator) => generator.visit_accounts(accounts),
        }
    }
}

pub trait ConstraintGenerator: ConstraintVisitor + Sized {
    fn generate(&self) -> Result<GeneratorResult, syn::Error>;
}

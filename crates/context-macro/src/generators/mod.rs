mod arguments;
mod assign;
mod bumps;
mod has_one;
mod init;
mod init_if_needed;
mod rent;
mod tokens_gen;

use {crate::StagedGenerator, proc_macro2::TokenStream, syn::Field};
pub use {arguments::*, assign::*, bumps::*, has_one::*, init::*, init_if_needed::*, rent::*};

#[derive(Default, Clone)]
pub struct GeneratorResult {
    pub outside: TokenStream,
    pub inside: TokenStream,
    pub new_fields: Vec<Field>,
}

pub enum ConstraintGenerators {
    HasOne(HasOneGenerator),
    Init(InitializationGenerator),
    Rent(RentGenerator),
    Args(ArgumentsGenerator),
    Assign(AssignGenerator),
    Bumps(Box<BumpsGenerator>),
    InitIfNeeded(InitIfNeededGenerator),
}

impl StagedGenerator for ConstraintGenerators {
    fn append(&mut self, context: &mut crate::GenerationContext) -> Result<(), syn::Error> {
        match self {
            ConstraintGenerators::HasOne(generator) => generator.append(context),
            ConstraintGenerators::Init(generator) => generator.append(context),
            ConstraintGenerators::Rent(generator) => generator.append(context),
            ConstraintGenerators::Args(generator) => generator.append(context),
            ConstraintGenerators::Assign(generator) => generator.append(context),
            ConstraintGenerators::Bumps(generator) => generator.append(context),
            ConstraintGenerators::InitIfNeeded(generator) => generator.append(context),
        }
    }
}

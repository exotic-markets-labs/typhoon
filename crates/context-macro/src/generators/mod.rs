mod arguments;
mod assign;
mod bumps;
mod has_one;
mod rent;
mod token;
mod tokens_gen;

use {crate::StagedGenerator, proc_macro2::TokenStream, syn::Field};
pub use {arguments::*, assign::*, bumps::*, has_one::*, rent::*, token::*};

#[derive(Default, Clone)]
pub struct GeneratorResult {
    pub outside: TokenStream,
    pub inside: TokenStream,
    pub new_fields: Vec<Field>,
}

pub enum ConstraintGenerators {
    HasOne(HasOneGenerator),
    Rent(RentGenerator),
    Args(ArgumentsGenerator),
    Assign(AssignGenerator),
    Bumps(BumpsGenerator),
    Token(TokenAccountGenerator),
}

impl StagedGenerator for ConstraintGenerators {
    fn append(&mut self, context: &mut crate::GenerationContext) -> Result<(), syn::Error> {
        match self {
            ConstraintGenerators::HasOne(generator) => generator.append(context),
            ConstraintGenerators::Rent(generator) => generator.append(context),
            ConstraintGenerators::Args(generator) => generator.append(context),
            ConstraintGenerators::Assign(generator) => generator.append(context),
            ConstraintGenerators::Bumps(generator) => generator.append(context),
            ConstraintGenerators::Token(generator) => generator.append(context),
        }
    }
}

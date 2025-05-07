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

pub enum ConstraintGenerators<'a> {
    HasOne(HasOneGenerator<'a>),
    Rent(RentGenerator<'a>),
    Args(ArgumentsGenerator<'a>),
    Assign(AssignGenerator<'a>),
    Bumps(BumpsGenerator<'a>),
    Token(TokenAccountGenerator<'a>),
}

impl StagedGenerator for ConstraintGenerators<'_> {
    fn append(&mut self, context: &mut GeneratorResult) -> Result<(), syn::Error> {
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

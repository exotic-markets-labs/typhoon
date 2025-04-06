use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

mod bump;
mod has_one;
mod init;
mod keys;
mod payer;
mod seeded;
mod seeds;
mod space;

pub use {bump::*, has_one::*, init::*, keys::*, payer::*, seeded::*, seeds::*, space::*};

pub const CONSTRAINT_IDENT_STR: &str = "constraint";

//TODO rewrite it to add custom constraint for users
#[derive(Clone)]
pub enum Constraint {
    Init(ConstraintInit),
    Payer(ConstraintPayer),
    Space(ConstraintSpace),
    Seeded(ConstraintSeeded),
    Keys(ConstraintKeys),
    Seeds(ConstraintSeeds),
    Bump(ConstraintBump),
    HasOne(ConstraintHasOne),
}

#[derive(Clone, Default)]
pub struct Constraints(pub Vec<Constraint>);

impl TryFrom<&Vec<syn::Attribute>> for Constraints {
    type Error = syn::Error;

    fn try_from(value: &Vec<syn::Attribute>) -> Result<Self, Self::Error> {
        let constraints = value
            .iter()
            .filter(|attr| attr.path().is_ident(CONSTRAINT_IDENT_STR))
            .map(|attr| attr.parse_args_with(parse_constraints))
            .collect::<Result<Vec<Vec<Constraint>>, syn::Error>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(Constraints(constraints))
    }
}

pub fn parse_constraints(input: ParseStream) -> syn::Result<Vec<Constraint>> {
    let mut constraints = Vec::new();

    while !input.is_empty() {
        let name = input.parse::<Ident>()?.to_string();
        match name.as_str() {
            "init" => {
                constraints.push(Constraint::Init(ConstraintInit));
            }
            "payer" => {
                constraints.push(Constraint::Payer(ConstraintPayer::parse(input)?));
            }
            "space" => {
                constraints.push(Constraint::Space(ConstraintSpace::parse(input)?));
            }
            "seeds" => {
                constraints.push(Constraint::Seeds(ConstraintSeeds::parse(input)?));
            }
            "bump" => {
                constraints.push(Constraint::Bump(ConstraintBump::parse(input)?));
            }
            "seeded" => {
                constraints.push(Constraint::Seeded(ConstraintSeeded));
            }
            "keys" => {
                constraints.push(Constraint::Keys(ConstraintKeys::parse(input)?));
            }
            "has_one" => constraints.push(Constraint::HasOne(ConstraintHasOne::parse(input)?)),
            _ => return Err(syn::Error::new(input.span(), "Unknow constraint.")),
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
    }

    Ok(constraints)
}

#[cfg(test)]
mod tests {
    use {super::*, syn::parse_quote};

    #[test]
    fn test_parse_constraints() {
        let attributes: Vec<syn::Attribute> = parse_quote! {
            #[constraint(
                has_one = account,
                seeds = [
                    b"seed".as_ref(),
                ],
                bump = counter.data()?.bump,
            )]
        };

        let constraints = Constraints::try_from(&attributes).unwrap();

        assert_eq!(constraints.0.len(), 3);
    }
}

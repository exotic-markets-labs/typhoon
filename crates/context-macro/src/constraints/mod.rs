use syn::{
    parse::{Parse, ParseStream},
    visit_mut::VisitMut,
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

impl VisitMut for Constraints {
    fn visit_attributes_mut(&mut self, attrs: &mut Vec<syn::Attribute>) {
        self.0.reserve(attrs.len());

        attrs.retain(|attr| {
            if !attr.path().is_ident("constraint") {
                return true;
            }

            if let Ok(mut constraints) = attr.parse_args_with(parse_constraints) {
                self.0.append(&mut constraints);
            }

            false
        });
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
        let mut attributes: Vec<syn::Attribute> = parse_quote! {
            #[constraint(
                has_one = account,
                seeds = [
                    b"seed".as_ref(),
                ],
                bump = counter.data()?.bump,
            )]
        };

        let mut constraints = Constraints::default();
        constraints.visit_attributes_mut(&mut attributes);

        assert!(attributes.is_empty());
        assert_eq!(constraints.0.len(), 3);
    }
}

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    visit_mut::VisitMut,
    Expr, Ident, Token,
};

mod bump;
mod init;
mod payer;
mod seeds;
mod space;

use {bump::*, init::*, payer::*, seeds::*, space::*};

//TODO rewrite it to add custom constraint for users
pub enum Constraint {
    Init(ConstraintInit),
    Payer(ConstraintPayer),
    Space(ConstraintSpace),
    Seeds(ConstraintSeeds),
    Bump(ConstraintBump),
}

#[derive(Default)]
pub struct Constraints(Vec<Constraint>);

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

impl Constraints {
    pub fn has_init(&self) -> bool {
        self.0.iter().any(|c| matches!(&c, Constraint::Init(_)))
    }

    pub fn get_payer(&self) -> Option<&Expr> {
        self.0.iter().find_map(|c| {
            if let Constraint::Payer(ConstraintPayer { target }) = c {
                Some(target)
            } else {
                None
            }
        })
    }

    pub fn get_space(&self) -> Option<&Expr> {
        self.0.iter().find_map(|c| {
            if let Constraint::Space(ConstraintSpace { space }) = c {
                Some(space)
            } else {
                None
            }
        })
    }

    pub fn get_seeds(&self) -> Option<&Punctuated<Expr, Token![,]>> {
        self.0.iter().find_map(|c| {
            if let Constraint::Seeds(ConstraintSeeds { seeds }) = c {
                Some(seeds)
            } else {
                None
            }
        })
    }

    pub fn get_bump(&self, account_name: &Ident) -> Option<Expr> {
        self.0.iter().find_map(|c| {
            if let Constraint::Bump(bump_constraint) = c {
                if bump_constraint.is_some() {
                    if let Some(bump) = &bump_constraint.bump {
                        Some(bump.clone())
                    } else {
                        syn::parse_str::<Expr>(&format!("{}_bump", account_name)).ok()
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
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
            _ => return Err(syn::Error::new(input.span(), "Unknow constraint.")),
        }

        if input.peek(Token![,]) {
            let _punct: Token![,] = input.parse()?;
        }
    }

    Ok(constraints)
}

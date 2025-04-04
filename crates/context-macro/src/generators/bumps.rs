use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{
        accounts::Account,
        constraints::{ConstraintBump, ConstraintKeys, ConstraintSeeded, ConstraintSeeds},
        extractor::InnerTyExtractor,
        visitor::ConstraintVisitor,
    },
    quote::{format_ident, quote},
    syn::{parse_quote, punctuated::Punctuated, visit::Visit, Expr, Ident, PathSegment, Token},
};

#[derive(Default)]
pub struct BumpsGenerator {
    context_name: String,
    account: Option<(Ident, PathSegment)>,
    bump: Option<Expr>,
    is_seeded: bool,
    seeds: Option<Punctuated<Expr, Token![,]>>,
    result: GeneratorResult,
    struct_fields: Vec<Ident>,
}

impl BumpsGenerator {
    pub fn new(context_name: impl ToString) -> Self {
        BumpsGenerator {
            context_name: context_name.to_string(),
            ..Default::default()
        }
    }

    pub fn is_pda(&self) -> bool {
        self.is_seeded || self.seeds.is_some()
    }

    fn extend_checks(&mut self) -> Result<(), syn::Error> {
        let (name, ty) = self.account.as_ref().unwrap();
        let pda_key = format_ident!("{}_key", name);
        let pda_bump = format_ident!("{}_bump", name);

        if let Some(bump) = &self.bump {
            let (seeds_token, bump_token) = if self.is_seeded {
                (
                    quote!(#name.data()?.seeds_with_bump(&[#pda_bump])),
                    quote!(let #pda_bump = { #bump };),
                )
            } else {
                let seeds = self.seeds.as_ref().ok_or(syn::Error::new(
                    name.span(),
                    "Seeds constraint is not specified.",
                ))?;
                (
                    quote!([#seeds, &[#pda_bump]]),
                    quote!(let #pda_bump = { #bump };),
                )
            };

            self.result.after_init.extend(quote! {
                #bump_token
                let #pda_key = create_program_address(&#seeds_token, &crate::ID)?;
                if #name.key() != &#pda_key {
                    return Err(ProgramError::InvalidSeeds);
                }
            });
        } else {
            let keys = self.seeds.as_ref().ok_or(syn::Error::new(
                name.span(),
                "Seeds constraint is not specified.",
            ))?;

            let seeds = if self.is_seeded {
                let mut inner_ty_extractor = InnerTyExtractor::new();
                inner_ty_extractor.visit_path_segment(ty);
                let inner_ty_str = inner_ty_extractor
                    .ty
                    .ok_or(syn::Error::new(name.span(), "Cannot find the inner type."))?;
                let inner_ty = format_ident!("{inner_ty_str}");

                quote!(#inner_ty::derive(#keys))
            } else {
                quote!([#keys])
            };

            self.result.at_init.extend(quote! {
                let (#pda_key, #pda_bump) = find_program_address(&#seeds, &crate::ID);
                if #name.key() != &#pda_key {
                    return Err(ProgramError::InvalidSeeds);
                }
            });
        }

        Ok(())
    }
}

impl ConstraintGenerator for BumpsGenerator {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        let mut result = self.result.clone();

        if !self.struct_fields.is_empty() {
            let struct_name = format_ident!("{}Bumps", self.context_name);
            let struct_fields = &self.struct_fields;
            let bumps_struct = quote! {
                #[derive(Debug, PartialEq)]
                pub struct #struct_name {
                    #(pub #struct_fields: u8,)*
                }
            };

            result.global_outside = bumps_struct;
            let assign_fields = self.struct_fields.iter().map(|n| {
                let bump_ident = format_ident!("{}_bump", n);
                quote!(#n: #bump_ident)
            });
            result.at_init.extend(quote! {
                let bumps = #struct_name {
                    #(#assign_fields),*
                };
            });

            result.new_fields.push(parse_quote! {
                pub bumps: #struct_name
            });
        }

        Ok(result)
    }
}

impl ConstraintVisitor for BumpsGenerator {
    fn visit_account(&mut self, account: &Account) -> Result<(), syn::Error> {
        self.account = Some((account.name.clone(), account.ty.clone()));
        self.bump = None;
        self.is_seeded = false;
        self.seeds = None;

        self.visit_constraints(&account.constraints)?;

        if self.is_pda() {
            self.extend_checks()?;
        }

        Ok(())
    }

    fn visit_bump(&mut self, constraint: &ConstraintBump) -> Result<(), syn::Error> {
        self.bump = constraint.0.clone();

        if self.bump.is_none() {
            self.struct_fields
                .push(self.account.as_ref().unwrap().0.clone());
        }

        Ok(())
    }

    fn visit_seeded(&mut self, _constraint: &ConstraintSeeded) -> Result<(), syn::Error> {
        self.is_seeded = true;

        //TODO add check seeds constraint

        Ok(())
    }

    fn visit_seeds(&mut self, constraint: &ConstraintSeeds) -> Result<(), syn::Error> {
        self.seeds = Some(constraint.seeds.clone());
        //TODO add check seeded constraint

        Ok(())
    }

    fn visit_keys(&mut self, constraint: &ConstraintKeys) -> Result<(), syn::Error> {
        self.seeds = Some(constraint.keys.clone());

        Ok(())
    }
}

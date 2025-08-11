use {
    crate::{accounts::Account, visitor::ContextVisitor},
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{parse_quote, Expr},
    typhoon_syn::{
        constraints::{
            ConstraintAssociatedToken, ConstraintBump, ConstraintInitIfNeeded, ConstraintProgram,
            ConstraintSeeded, ConstraintSeeds,
        },
        utils::{ContextExpr, SeedsExpr},
    },
};

pub enum PdaType {
    Default,
    Seeded,
    Ata,
}

pub struct BumpTokenGenerator<'a> {
    account: &'a Account,
    init_if_needed: bool,
    pda_ty: PdaType,
    seeds: Option<SeedsExpr>,
    program_id: Option<Expr>,
    bump: Option<ContextExpr>,
}

impl<'a> BumpTokenGenerator<'a> {
    pub fn new(account: &'a Account) -> Self {
        BumpTokenGenerator {
            account,
            pda_ty: PdaType::Default,
            seeds: None,
            program_id: None,
            bump: None,
            init_if_needed: false,
        }
    }

    fn seeds_without_bump(&self) -> Result<TokenStream, syn::Error> {
        let keys = self.seeds.as_ref().ok_or(syn::Error::new(
            self.account.name.span(),
            "Seeds constraint is not specified.",
        ))?;

        let seeds = if matches!(self.pda_ty, PdaType::Seeded) {
            let inner_ty = format_ident!("{}", self.account.inner_ty);

            quote!(#inner_ty::derive(#keys))
        } else {
            match keys {
                SeedsExpr::Punctuated(punctuated) => quote!([#punctuated]),
                SeedsExpr::Single(expr) => quote!(#expr),
            }
        };
        Ok(seeds)
    }

    pub fn generate(self) -> Result<(TokenStream, Option<TokenStream>, TokenStream), syn::Error> {
        let name = &self.account.name;
        let name_str = name.to_string();
        let pda_key = format_ident!("{}_key", name);
        let pda_bump = format_ident!("{}_bump", name);
        let program_id = self
            .program_id
            .as_ref()
            .map(|p| quote!(#p))
            .unwrap_or(quote!(program_id));

        let (pda, pda_no_bump) = if let Some(bump) = &self.bump {
            let var_name = format_ident!("{name}_state");
            let seeds_token = if matches!(self.pda_ty, PdaType::Seeded) {
                quote!(#var_name.seeds_with_bump(&[#pda_bump]))
            } else {
                let seeds = self.seeds.as_ref().ok_or(syn::Error::new(
                    name.span(),
                    "Seeds constraint is not specified.",
                ))?;
                quote!([#seeds, &[#pda_bump]])
            };
            let seeds_without_bump = if self.init_if_needed {
                let seed_token = self.seeds_without_bump()?;
                Some(
                    quote!(let (#pda_key, #pda_bump) = find_program_address(&#seed_token, &#program_id);),
                )
            } else {
                None
            };

            let (state_assign, drop) = if bump.name().is_some() && self.init_if_needed {
                (
                    Some(quote!(let #var_name = #name.data()?;)),
                    Some(quote!(drop(#var_name);)),
                )
            } else {
                (None, None)
            };

            (
                quote! {
                    #state_assign
                    let #pda_bump = #bump;
                    let #pda_key = create_program_address(&#seeds_token, &#program_id)?;
                    #drop
                },
                seeds_without_bump,
            )
        } else {
            let seeds = self.seeds_without_bump()?;

            (
                quote! {
                    let (#pda_key, #pda_bump) = find_program_address(&#seeds, &#program_id);
                },
                Some(quote! {
                    let (#pda_key, #pda_bump) = find_program_address(&#seeds, &#program_id);
                }),
            )
        };

        Ok((
            pda,
            pda_no_bump,
            quote! {
                if #name.key() != &#pda_key {
                    return Err(Error::new(ProgramError::InvalidSeeds).with_account(#name_str));
                }
            },
        ))
    }
}

impl ContextVisitor for BumpTokenGenerator<'_> {
    fn visit_program(&mut self, constraint: &ConstraintProgram) -> Result<(), syn::Error> {
        self.program_id = Some(constraint.0.clone());

        Ok(())
    }

    fn visit_bump(&mut self, constraint: &ConstraintBump) -> Result<(), syn::Error> {
        self.bump = constraint.0.clone();

        Ok(())
    }

    fn visit_seeded(&mut self, constraint: &ConstraintSeeded) -> Result<(), syn::Error> {
        if matches!(self.pda_ty, PdaType::Default | PdaType::Ata) && self.seeds.is_some() {
            return Err(syn::Error::new(
                self.account.name.span(),
                "Cannot specified keys and seeds at the same time.",
            ));
        }

        self.pda_ty = PdaType::Seeded;
        self.seeds = constraint.0.clone();

        Ok(())
    }

    fn visit_seeds(&mut self, constraint: &ConstraintSeeds) -> Result<(), syn::Error> {
        if matches!(self.pda_ty, PdaType::Seeded | PdaType::Ata) {
            return Err(syn::Error::new(
                self.account.name.span(),
                "Cannot specified keys and seeds at the same time.",
            ));
        } //TODO change it in cross check

        self.seeds = Some(constraint.seeds.clone());

        Ok(())
    }

    fn visit_init_if_needed(
        &mut self,
        _constraint: &ConstraintInitIfNeeded,
    ) -> Result<(), syn::Error> {
        self.init_if_needed = true;

        Ok(())
    }

    fn visit_associated_token(
        &mut self,
        constraint: &ConstraintAssociatedToken,
    ) -> Result<(), syn::Error> {
        self.pda_ty = PdaType::Ata;

        match constraint {
            ConstraintAssociatedToken::Mint(ident) => {
                if let Some(seeds) = self.seeds.as_mut() {
                    let SeedsExpr::Punctuated(expr) = seeds else {
                        return Err(syn::Error::new_spanned(
                            &seeds,
                            "Seeds expr cannot be used in this context.",
                        ));
                    };

                    expr.insert(2, parse_quote!(#ident));
                } else {
                    self.seeds = Some(SeedsExpr::Punctuated(
                        parse_quote!(token_program.key().as_ref(), #ident),
                    ));
                    self.program_id = Some(parse_quote!(AtaTokenProgram::ID))
                }
            }
            ConstraintAssociatedToken::Authority(ident) => {
                if let Some(seeds) = self.seeds.as_mut() {
                    let SeedsExpr::Punctuated(expr) = seeds else {
                        return Err(syn::Error::new_spanned(
                            &seeds,
                            "Seeds expr cannot be used in this context.",
                        ));
                    };

                    expr.insert(0, parse_quote!(#ident));
                } else {
                    self.seeds = Some(SeedsExpr::Punctuated(
                        parse_quote!(#ident, token_program.key().as_ref()),
                    ));
                    self.program_id = Some(parse_quote!(AtaTokenProgram::ID))
                }
            }
        }

        Ok(())
    }
}

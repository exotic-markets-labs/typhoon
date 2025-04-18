use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{
        accounts::Account,
        constraints::{
            ConstraintAssociatedToken, ConstraintBump, ConstraintInitIfNeeded, ConstraintProgram,
            ConstraintSeeded, ConstraintSeeds,
        },
        context::Context,
        visitor::ContextVisitor,
    },
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{parse_quote, punctuated::Punctuated, Expr, Ident, Token},
};

pub enum PdaType {
    Default,
    Seeded,
    Ata,
}

pub struct PdaGenerator<'a> {
    account: &'a Account,
    init_if_needed: bool,
    pda_ty: PdaType,
    seeds: Option<Punctuated<Expr, Token![,]>>,
    program_id: Option<Expr>,
    bump: Option<Expr>,
}

impl<'a> PdaGenerator<'a> {
    pub fn new(account: &'a Account) -> Self {
        PdaGenerator {
            account,
            pda_ty: PdaType::Default,
            seeds: None,
            program_id: None,
            bump: None,
            init_if_needed: false,
        }
    }

    pub fn generate(self) -> Result<(TokenStream, bool), syn::Error> {
        let name = &self.account.name;
        let pda_key = format_ident!("{}_key", name);
        let pda_bump = format_ident!("{}_bump", name);
        let program_id = self
            .program_id
            .as_ref()
            .map(|p| quote!(#p))
            .unwrap_or(quote!(crate::ID));

        let (pda, seeds) = if let Some(bump) = &self.bump {
            let seeds_token = if matches!(self.pda_ty, PdaType::Seeded) {
                quote!(#name.data()?.seeds_with_bump(&[#pda_bump]))
            } else {
                let seeds = self.seeds.as_ref().ok_or(syn::Error::new(
                    name.span(),
                    "Seeds constraint is not specified.",
                ))?;
                quote!([#seeds, &[#pda_bump]])
            };

            (
                quote! {
                    let #pda_bump = { #bump };
                    let #pda_key = create_program_address(&#seeds_token, &#program_id)?;
                },
                seeds_token,
            )
        } else {
            let keys = self.seeds.as_ref().ok_or(syn::Error::new(
                name.span(),
                "Seeds constraint is not specified.",
            ))?;

            let seeds = if matches!(self.pda_ty, PdaType::Seeded) {
                let inner_ty = format_ident!("{}", self.account.inner_ty);

                quote!(#inner_ty::derive(#keys))
            } else {
                quote!([#keys])
            };

            (
                quote! {
                    let (#pda_key, #pda_bump) = find_program_address(&#seeds, &#program_id);
                },
                seeds,
            )
        };

        let pda_assign = if self.init_if_needed {
            quote! {
                let (#pda_key, #pda_bump) = if <Mut<UncheckedAccount> as ChecksExt>::is_initialized(&#name) {
                    #pda
                    (#pda_key, #pda_bump)
                }else {
                    find_program_address(&#seeds, &#program_id)
                };
            }
        } else {
            quote!(#pda)
        };

        Ok((
            quote! {
                #pda_assign
                if #name.key() != &#pda_key {
                    return Err(ProgramError::InvalidSeeds);
                }
            },
            self.bump.is_none() || self.init_if_needed,
        ))
    }
}

impl ContextVisitor for PdaGenerator<'_> {
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
                    seeds.insert(2, parse_quote!(#ident));
                } else {
                    self.seeds = Some(parse_quote!(token_program.key().as_ref(), #ident));
                    self.program_id = Some(parse_quote!(AtaTokenProgram::ID))
                }
            }
            ConstraintAssociatedToken::Authority(ident) => {
                if let Some(seeds) = self.seeds.as_mut() {
                    seeds.insert(0, parse_quote!(#ident));
                } else {
                    self.seeds = Some(parse_quote!(#ident, token_program.key().as_ref()));
                    self.program_id = Some(parse_quote!(AtaTokenProgram::ID))
                }
            }
        }

        Ok(())
    }
}

// TODO change to add builder by account
#[derive(Default)]
pub struct BumpsGenerator {
    context_name: Option<String>,
    is_pda: bool,
    result: GeneratorResult,
    struct_fields: Vec<Ident>,
}

impl BumpsGenerator {
    pub fn new() -> Self {
        BumpsGenerator::default()
    }
}

impl ConstraintGenerator for BumpsGenerator {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        let mut result = self.result.clone();

        if !self.struct_fields.is_empty() {
            let struct_name = format_ident!("{}Bumps", self.context_name.as_ref().unwrap());
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

impl ContextVisitor for BumpsGenerator {
    fn visit_context(&mut self, context: &Context) -> Result<(), syn::Error> {
        self.context_name = Some(context.item_struct.ident.to_string());
        self.visit_accounts(&context.accounts)?;

        Ok(())
    }

    fn visit_bump(&mut self, _constraint: &ConstraintBump) -> Result<(), syn::Error> {
        self.is_pda = true;

        Ok(())
    }

    fn visit_account(&mut self, account: &Account) -> Result<(), syn::Error> {
        self.visit_constraints(&account.constraints)?;

        let mut pda_generator = PdaGenerator::new(account);
        pda_generator.visit_account(account)?;

        if self.is_pda {
            let (token, is_field_generated) = pda_generator.generate()?;

            if is_field_generated {
                self.struct_fields.push(account.name.clone());
            }
            self.result.at_init.extend(token);

            self.is_pda = false;
        }

        Ok(())
    }
}

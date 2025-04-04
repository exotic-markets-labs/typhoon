use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{
        accounts::{Account, Accounts},
        constraints::{
            ConstraintInit, ConstraintKeys, ConstraintPayer, ConstraintSeeds, ConstraintSpace,
        },
        extractor::InnerTyExtractor,
        visitor::ConstraintVisitor,
    },
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote},
    syn::{punctuated::Punctuated, visit::Visit, Expr, Ident, PathSegment, Token},
};

#[derive(Default)]
pub struct InitializationGenerator {
    account: Option<(Ident, PathSegment)>,
    need_check: bool,
    keys: Option<Punctuated<Expr, Token![,]>>,
    space: Option<Expr>,
    payer: Option<Ident>,
    is_seeded: bool,
    has_init: bool,
    result: TokenStream,
}

impl InitializationGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_span(&self) -> Span {
        self.account
            .as_ref()
            .map(|acc| acc.0.span())
            .unwrap_or(Span::call_site())
    }

    fn get_seeds(&self) -> Option<TokenStream> {
        let punctuated_keys = self.keys.as_ref()?;

        let seeds = if self.is_seeded {
            let mut inner_ty_extractor = InnerTyExtractor::new();
            inner_ty_extractor.visit_path_segment(&self.account.as_ref()?.1);
            let inner_ty = inner_ty_extractor.ty?;
            let account_ty = format_ident!("{}", inner_ty);

            quote!(#account_ty::derive_with_bump(#punctuated_keys, &bump))
        } else {
            quote!(seeds!(#punctuated_keys, &bump))
        };

        Some(seeds)
    }

    fn check_prerequisite(&self, context: &[Account]) -> Result<(), syn::Error> {
        let has_system_program = context.iter().any(|acc| {
            let mut extractor = InnerTyExtractor::new();
            extractor.visit_path_arguments(&acc.ty.arguments);
            acc.ty.ident == "Program" && extractor.ty.as_deref() == Some("System")
        });

        if !has_system_program {
            return Err(syn::Error::new(
                self.get_span(),
                "Using `init` requires including the `Program<System>` account",
            ));
        }

        Ok(())
    }

    fn extend_result(&mut self) -> Result<(), syn::Error> {
        let (name, ty) = self.account.as_ref().unwrap();
        let expanded = if self.has_init {
            let Some(ref payer) = self.payer else {
                return Err(syn::Error::new(
                    self.get_span(),
                    "A payer is needed for the `init` constraint",
                ));
            };
            let Some(ref space) = self.space else {
                return Err(syn::Error::new(
                    self.get_span(),
                    "A space need to be specified for the `init` constraint",
                ));
            };

            if let Some(seeds) = self.get_seeds() {
                quote! {
                    let #name: #ty = {
                        let system_acc = <typhoon::lib::Mut<typhoon::lib::SystemAccount> as typhoon::lib::FromAccountInfo>::try_from_info(#name)?;
                        // TODO: avoid reusing seeds here and in verifications
                        let bump = [bumps.#name];
                        let seeds = #seeds;
                        let signer = instruction::CpiSigner::from(&seeds);
                        typhoon::lib::SystemCpi::create_account(system_acc, &rent, &#payer, &crate::ID, #space, Some(&[signer]))?
                    };
                }
            } else {
                quote! {
                    let #name: #ty = {
                        let system_acc = <typhoon::lib::Mut<typhoon::lib::SystemAccount> as typhoon::lib::FromAccountInfo>::try_from_info(#name)?;
                        typhoon::lib::SystemCpi::create_account(system_acc, &rent, &#payer, &crate::ID, #space, None)?
                    };
                }
            }
        } else {
            quote! {
                let #name = <#ty as FromAccountInfo>::try_from_info(#name)?;
            }
        };

        self.result.extend(expanded);

        Ok(())
    }
}

impl ConstraintGenerator for InitializationGenerator {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        Ok(GeneratorResult {
            at_init: self.result.clone(),
            ..Default::default()
        })
    }
}

impl ConstraintVisitor for InitializationGenerator {
    fn visit_init(&mut self, _constraint: &ConstraintInit) -> Result<(), syn::Error> {
        self.has_init = true;
        self.need_check = true;

        Ok(())
    }

    fn visit_accounts(&mut self, accounts: &Accounts) -> Result<(), syn::Error> {
        for account in &accounts.0 {
            self.visit_account(account)?;
        }

        if self.need_check {
            self.check_prerequisite(&accounts.0)?;
        }

        Ok(())
    }

    fn visit_account(&mut self, account: &Account) -> Result<(), syn::Error> {
        *self = Self {
            account: Some((account.name.clone(), account.ty.clone())),
            need_check: self.need_check,
            result: self.result.clone(),
            ..Default::default()
        };
        self.visit_constraints(&account.constraints)?;
        self.extend_result()
    }

    fn visit_payer(&mut self, contraint: &ConstraintPayer) -> Result<(), syn::Error> {
        self.payer = Some(contraint.target.clone());

        Ok(())
    }

    fn visit_space(&mut self, contraint: &ConstraintSpace) -> Result<(), syn::Error> {
        self.space = Some(contraint.space.clone());

        Ok(())
    }

    fn visit_keys(&mut self, contraint: &ConstraintKeys) -> Result<(), syn::Error> {
        if !self.is_seeded && self.keys.is_some() {
            return Err(syn::Error::new(
                self.get_span(),
                "Cannot specified keys and seeds at the same time.",
            ));
        }

        self.keys = Some(contraint.keys.clone());
        self.is_seeded = true;

        Ok(())
    }

    fn visit_seeds(&mut self, contraint: &ConstraintSeeds) -> Result<(), syn::Error> {
        if self.is_seeded {
            return Err(syn::Error::new(
                self.get_span(),
                "Cannot specified keys and seeds at the same time.",
            ));
        }

        self.keys = Some(contraint.seeds.clone());

        Ok(())
    }
}

use {
    super::{tokens_gen::BumpTokenGenerator, GeneratorResult},
    crate::{
        constraints::{ConstraintBump, ConstraintInitIfNeeded},
        context::Context,
        generators::tokens_gen::InitTokenGenerator,
        visitor::ContextVisitor,
        StagedGenerator,
    },
    quote::{format_ident, quote},
    syn::Ident,
};

#[derive(Default)]
struct Checks {
    has_bump: bool,
    bump_name: Option<Ident>,
    has_init_if_needed: bool,
}

impl Checks {
    pub fn new() -> Self {
        Checks::default()
    }
}

impl ContextVisitor for Checks {
    fn visit_init_if_needed(
        &mut self,
        _constraint: &ConstraintInitIfNeeded,
    ) -> Result<(), syn::Error> {
        self.has_init_if_needed = true;
        Ok(())
    }

    fn visit_bump(&mut self, constraint: &ConstraintBump) -> Result<(), syn::Error> {
        self.has_bump = true;
        if let Some(ref e) = constraint.0 {
            self.bump_name = e.name().cloned();
        }
        Ok(())
    }
}

pub struct InitIfNeededGenerator<'a>(&'a Context);

impl<'a> InitIfNeededGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self(context)
    }
}

impl StagedGenerator for InitIfNeededGenerator<'_> {
    fn append(&mut self, result: &mut GeneratorResult) -> Result<(), syn::Error> {
        for account in &self.0.accounts {
            let mut checks = Checks::new();
            checks.visit_account(account)?;

            if checks.has_init_if_needed {
                let name = &account.name;
                let account_ty = &account.ty;

                let mut init_gen = InitTokenGenerator::new(account);
                init_gen.visit_account(account)?;
                let init_token = init_gen.generate()?;

                if checks.has_bump {
                    let pda_key = format_ident!("{}_key", name);
                    let pda_bump = format_ident!("{}_bump", name);
                    let mut bump_gen = BumpTokenGenerator::new(account);
                    bump_gen.visit_account(account)?;
                    let (pda_token, find_pda_token, check_token) = bump_gen.generate()?;

                    result.inside.extend(quote! {
                        let (#name, #pda_key, #pda_bump) = if !#name.is_owned_by(&Pubkey::default()) {
                            let #name = <#account_ty as FromAccountInfo>::try_from_info(#name.into())?;
                            #pda_token
                            (#name, #pda_key, #pda_bump)
                        }else {
                            #find_pda_token
                            let #name = { #init_token };
                            (#name, #pda_key, #pda_bump)
                        };
                        #check_token
                    });
                } else {
                    result.inside.extend(quote! {
                            let #name = if !#name.is_owned_by(&Pubkey::default()) {
                                <#account_ty as FromAccountInfo>::try_from_info(#name.into())?
                            }else {
                                #init_token
                            };
                    });
                }
            }
        }

        Ok(())
    }
}

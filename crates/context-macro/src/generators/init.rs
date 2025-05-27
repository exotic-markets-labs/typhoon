use {
    super::tokens_gen::{BumpTokenGenerator, InitTokenGenerator, StateTokenGenerator},
    crate::{
        accounts::Account,
        constraints::{ConstraintBump, ConstraintInit, ConstraintInitIfNeeded},
        context::Context,
        visitor::ContextVisitor,
        StagedGenerator,
    },
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
};

#[derive(Default)]
struct Checks {
    has_bump: bool,
    has_init: bool,
    has_init_if_needed: bool,
}

impl Checks {
    pub fn new() -> Self {
        Checks::default()
    }
}

impl ContextVisitor for Checks {
    fn visit_bump(&mut self, _constraint: &ConstraintBump) -> Result<(), syn::Error> {
        self.has_bump = true;
        Ok(())
    }

    fn visit_init(&mut self, _constraint: &ConstraintInit) -> Result<(), syn::Error> {
        self.has_init = true;
        Ok(())
    }

    fn visit_init_if_needed(
        &mut self,
        _constraint: &ConstraintInitIfNeeded,
    ) -> Result<(), syn::Error> {
        self.has_init_if_needed = true;
        Ok(())
    }
}

pub struct InitGenerator<'a>(&'a Context);

impl<'a> InitGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self(context)
    }

    pub fn generate_init_if_needed(
        &self,
        account: &Account,
        has_bump: bool,
    ) -> Result<TokenStream, syn::Error> {
        let name = &account.name;
        let account_ty = &account.ty;

        let mut init_gen = InitTokenGenerator::new(account);
        init_gen.visit_account(account)?;
        let init_token = init_gen.generate()?;

        let expanded = if has_bump {
            let pda_key = format_ident!("{}_key", name);
            let pda_bump = format_ident!("{}_bump", name);
            let mut bump_gen = BumpTokenGenerator::new(account);
            bump_gen.visit_account(account)?;
            let (pda_token, find_pda_token, check_token) = bump_gen.generate()?;

            quote! {
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
            }
        } else {
            quote! {
                    let #name = if !#name.is_owned_by(&Pubkey::default()) {
                        <#account_ty as FromAccountInfo>::try_from_info(#name.into())?
                    }else {
                        #init_token
                    };
            }
        };

        Ok(expanded)
    }
}

impl StagedGenerator for InitGenerator<'_> {
    fn append(&mut self, result: &mut super::GeneratorResult) -> Result<(), syn::Error> {
        let state_gen = StateTokenGenerator::analyze(self.0)?;
        for account in &self.0.accounts {
            let mut checks = Checks::new();
            checks.visit_account(account)?;

            let maybe_state = state_gen.get_token(&account.name);

            if checks.has_init_if_needed {
                let token = self.generate_init_if_needed(account, checks.has_bump)?;
                result.inside.extend(token);

                if let Some((token, var_name)) = maybe_state {
                    result.inside.extend(token);
                    result.drop_vars.push(var_name);
                }

                continue;
            }

            if let Some((token, var_name)) = maybe_state {
                result.inside.extend(token);
                result.drop_vars.push(var_name);
            }

            if checks.has_bump {
                let mut pda_generator = BumpTokenGenerator::new(account);
                pda_generator.visit_account(account)?;

                let (pda, _, check) = pda_generator.generate()?;

                result.inside.extend(quote! {
                    #pda
                    #check
                });
            }

            if checks.has_init {
                let name = &account.name;
                let account_ty = &account.ty;
                let mut init_gen = InitTokenGenerator::new(account);
                init_gen.visit_account(account)?;
                let init_token = init_gen.generate()?;

                result.inside.extend(quote! {
                    let #name: #account_ty = {
                        #init_token
                    };
                });
            }
        }

        Ok(())
    }
}

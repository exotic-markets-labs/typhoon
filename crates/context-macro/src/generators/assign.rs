use {
    crate::{
        accounts::Account,
        constraints::{ConstraintInit, ConstraintInitIfNeeded},
        visitor::ContextVisitor,
        GenerationContext, StagedGenerator,
    },
    proc_macro2::TokenStream,
    quote::quote,
};

struct AccountGenerator<'a> {
    account: &'a Account,
    has_init: bool,
    has_init_if_needed: bool,
}

impl<'a> AccountGenerator<'a> {
    pub fn new(account: &'a Account) -> Self {
        Self {
            account,
            has_init: false,
            has_init_if_needed: false,
        }
    }

    pub fn generate(&self) -> Result<TokenStream, syn::Error> {
        let name = &self.account.name;
        if self.has_init {
            Ok(quote! {
                let #name = <Mut<SystemAccount> as FromAccountInfo>::try_from_info(#name)?;
            })
        } else if self.has_init_if_needed {
            Ok(quote! {
                let #name = <Mut<UncheckedAccount> as FromAccountInfo>::try_from_info(#name)?;
            })
        } else {
            let account_ty = &self.account.ty;
            Ok(quote! {
                let #name = <#account_ty as FromAccountInfo>::try_from_info(#name)?;
            })
        }
    }
}

impl ContextVisitor for AccountGenerator<'_> {
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

pub struct AssignGenerator;

impl AssignGenerator {
    pub fn new() -> Self {
        AssignGenerator
    }
}

impl StagedGenerator for AssignGenerator {
    fn append(&mut self, context: &mut GenerationContext) -> Result<(), syn::Error> {
        for account in &context.input.accounts {
            let mut generator = AccountGenerator::new(account);
            generator.visit_account(account)?;
            context
                .generated_results
                .inside
                .extend(generator.generate()?);
        }

        Ok(())
    }
}

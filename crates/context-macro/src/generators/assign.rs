use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{accounts::Account, constraints::ConstraintInit, visitor::ContextVisitor},
    proc_macro2::TokenStream,
    quote::quote,
};

struct AccountGenerator<'a> {
    account: &'a Account,
    has_init: bool,
}

impl<'a> AccountGenerator<'a> {
    pub fn new(account: &'a Account) -> Self {
        Self {
            account,
            has_init: false,
        }
    }

    pub fn generate(&self) -> Result<TokenStream, syn::Error> {
        let name = &self.account.name;
        if self.has_init {
            Ok(quote! {
                let #name = <Mut<SystemAccount> as FromAccountInfo>::try_from_info(#name)?;
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
}

#[derive(Default)]
pub struct AssignGenerator {
    result: GeneratorResult,
}

impl AssignGenerator {
    pub fn new() -> Self {
        AssignGenerator::default()
    }
}

impl ConstraintGenerator for AssignGenerator {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        Ok(self.result.clone())
    }
}

impl ContextVisitor for AssignGenerator {
    fn visit_account(&mut self, account: &Account) -> Result<(), syn::Error> {
        let mut generator = AccountGenerator::new(account);
        generator.visit_account(account)?;
        let token = generator.generate()?;
        self.result.at_init.extend(token);

        Ok(())
    }
}

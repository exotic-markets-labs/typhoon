use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{
        accounts::Account,
        constraints::{ConstraintInit, ConstraintInitIfNeeded},
        visitor::ContextVisitor,
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

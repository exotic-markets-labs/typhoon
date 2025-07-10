use {
    super::GeneratorResult,
    crate::{accounts::Account, context::Context, visitor::ContextVisitor, StagedGenerator},
    proc_macro2::TokenStream,
    quote::quote,
    typhoon_syn::constraints::{ConstraintInit, ConstraintInitIfNeeded},
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

    pub fn generate(&self) -> Result<Option<TokenStream>, syn::Error> {
        let name = &self.account.name;
        let name_str = name.to_string();

        let assign = if self.has_init || self.has_init_if_needed {
            None
        } else {
            let account_ty = &self.account.ty;
            Some(quote! {
                <#account_ty as FromAccountInfo>::try_from_info(#name).trace_account(#name_str)?
            })
        };

        if assign.is_none() {
            return Ok(None);
        }

        if self.account.is_optional {
            Ok(Some(quote! {
                let #name = if #name.key() == program_id {
                    None
                } else {
                    Some(#assign)
                };
            }))
        } else {
            Ok(Some(quote!(let #name = #assign;)))
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

pub struct AssignGenerator<'a>(&'a Context);

impl<'a> AssignGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        AssignGenerator(context)
    }
}

impl StagedGenerator for AssignGenerator<'_> {
    fn append(&mut self, context: &mut GeneratorResult) -> Result<(), syn::Error> {
        for account in &self.0.accounts {
            let mut generator = AccountGenerator::new(account);
            generator.visit_account(account)?;
            context.inside.extend(generator.generate()?);
        }

        Ok(())
    }
}

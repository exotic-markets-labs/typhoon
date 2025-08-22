use {
    super::GeneratorResult,
    crate::{accounts::Account, context::Context, visitor::ContextVisitor, StagedGenerator},
    proc_macro2::TokenStream,
    quote::quote,
    syn::Ident,
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
            if account.is_array {
                // Handle array accounts by generating individual account processing for each element
                if let Some(size) = account.array_size {
                    let mut generator = AccountGenerator::new(account);
                    generator.visit_account(account)?;

                    for i in 0..size {
                        let element_name =
                            Ident::new(&format!("{}_{}", account.name, i), account.name.span());
                        let element_account = Account {
                            name: element_name.clone(),
                            constraints: account.constraints.clone(),
                            ty: account.ty.clone(),
                            is_optional: account.is_optional,
                            inner_ty: account.inner_ty.clone(),
                            is_array: false,
                            array_size: None,
                        };

                        let mut element_generator = AccountGenerator::new(&element_account);
                        element_generator.visit_account(&element_account)?;

                        if let Some(code) = element_generator.generate()? {
                            context.inside.extend(Some(code));
                        }
                    }

                    // Generate the array construction
                    let array_name = &account.name;
                    let element_names: Vec<Ident> = (0..size)
                        .map(|i| {
                            Ident::new(&format!("{}_{}", account.name, i), account.name.span())
                        })
                        .collect();

                    let array_code = quote! {
                        let #array_name = [#(#element_names),*];
                    };
                    context.inside.extend(Some(array_code));
                }
            } else {
                // Handle regular (non-array) accounts
                let mut generator = AccountGenerator::new(account);
                generator.visit_account(account)?;
                context.inside.extend(generator.generate()?);
            }
        }

        Ok(())
    }
}

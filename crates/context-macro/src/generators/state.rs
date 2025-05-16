use {
    super::GeneratorResult,
    crate::{
        constraints::{
            ConstraintBump, ConstraintHasOne, ConstraintInit, ConstraintInitIfNeeded,
            ConstraintToken,
        },
        context::Context,
        visitor::ContextVisitor,
        StagedGenerator,
    },
    quote::{format_ident, quote},
    std::collections::HashSet,
    syn::Ident,
};

struct TokenCheck {
    has_token: bool,
    has_init: bool,
}

impl TokenCheck {
    pub fn new() -> Self {
        TokenCheck {
            has_token: false,
            has_init: false,
        }
    }
}

impl ContextVisitor for TokenCheck {
    fn visit_init(&mut self, _constraint: &ConstraintInit) -> Result<(), syn::Error> {
        self.has_init = true;
        Ok(())
    }

    fn visit_init_if_needed(
        &mut self,
        _constraint: &ConstraintInitIfNeeded,
    ) -> Result<(), syn::Error> {
        self.has_init = true;
        Ok(())
    }

    fn visit_token(&mut self, _constraint: &ConstraintToken) -> Result<(), syn::Error> {
        self.has_token = true;
        Ok(())
    }
}

pub struct StateGenerator<'a> {
    context: &'a Context,
    state: HashSet<Ident>,
    current_account: Option<&'a Ident>,
}

impl<'a> StateGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        StateGenerator {
            context,
            state: HashSet::new(),
            current_account: None,
        }
    }
}

impl StagedGenerator for StateGenerator<'_> {
    fn append(&mut self, result: &mut GeneratorResult) -> Result<(), syn::Error> {
        for account in &self.context.accounts {
            self.current_account = Some(&account.name);

            let mut token_check = TokenCheck::new();
            token_check.visit_account(account)?;

            if token_check.has_token && !token_check.has_init {
                self.state.insert(account.name.to_owned());
            }

            self.visit_account(account)?;
        }

        let tokens = self.state.drain().map(|name| {
            let var_name = format_ident!("{name}_state");
            let token = quote!(let #var_name = #name.data()?;);
            result.drop_vars.push(var_name);
            token
        });

        result.inside.extend(tokens);

        Ok(())
    }
}

impl ContextVisitor for StateGenerator<'_> {
    fn visit_has_one(&mut self, _constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        let current_account = self.current_account.ok_or(syn::Error::new_spanned(
            &self.context.item_struct,
            "Not in account context",
        ))?;

        self.state.insert(current_account.to_owned());
        Ok(())
    }

    fn visit_bump(&mut self, constraint: &ConstraintBump) -> Result<(), syn::Error> {
        if let Some(c) = &constraint.0 {
            if let Some(name) = c.name() {
                self.state.insert(name.clone());
            }
        }

        Ok(())
    }
}

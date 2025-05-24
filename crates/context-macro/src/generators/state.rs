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
    std::collections::HashMap,
    syn::Ident,
};

struct Checks {
    has_token: bool,
    has_init: bool,
    has_init_if_needed: bool,
    has_one: bool,
}

impl Checks {
    pub fn new() -> Self {
        Checks {
            has_token: false,
            has_init: false,
            has_init_if_needed: false,
            has_one: false,
        }
    }
}

impl ContextVisitor for Checks {
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

    fn visit_token(&mut self, _constraint: &ConstraintToken) -> Result<(), syn::Error> {
        self.has_token = true;
        Ok(())
    }

    fn visit_has_one(&mut self, _constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        self.has_one = true;
        Ok(())
    }
}

pub struct StateGenerator<'a> {
    context: &'a Context,
    state: HashMap<Ident, bool>,
    current_account: Option<&'a Ident>,
}

impl<'a> StateGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        StateGenerator {
            context,
            state: HashMap::new(),
            current_account: None,
        }
    }
}

impl StagedGenerator for StateGenerator<'_> {
    fn append(&mut self, result: &mut GeneratorResult) -> Result<(), syn::Error> {
        let mut account_checks: HashMap<&Ident, (bool, bool)> = HashMap::new();
        for account in &self.context.accounts {
            self.current_account = Some(&account.name);

            let mut checks = Checks::new();
            checks.visit_account(account)?;

            account_checks.insert(&account.name, (checks.has_init_if_needed, checks.has_one));

            self.visit_account(account)?;

            if checks.has_token
                && !checks.has_init
                && !checks.has_init_if_needed
                && !self.state.contains_key(&account.name)
            {
                self.state.insert(account.name.to_owned(), false);
            }
        }

        for (name, has_bump) in self.state.drain() {
            let var_name = format_ident!("{name}_state");

            let Some((has_init_if_needed, has_one)) = account_checks.get(&name) else {
                continue;
            };

            if has_bump && *has_init_if_needed && !*has_one {
                continue;
            };

            let token = quote!(let #var_name = #name.data()?;);
            result.inside.extend(token);
            result.drop_vars.push(var_name);
        }

        Ok(())
    }
}

impl ContextVisitor for StateGenerator<'_> {
    fn visit_has_one(&mut self, _constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        let account_name = self.current_account.unwrap();
        if !self.state.contains_key(account_name) {
            self.state.insert(account_name.clone(), false);
        }

        Ok(())
    }

    fn visit_bump(&mut self, constraint: &ConstraintBump) -> Result<(), syn::Error> {
        if let Some(c) = &constraint.0 {
            if let Some(name) = c.name() {
                self.state.insert(name.clone(), true);
            }
        }

        Ok(())
    }
}

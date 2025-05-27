use {
    crate::{
        constraints::{
            ConstraintBump, ConstraintHasOne, ConstraintInit, ConstraintInitIfNeeded,
            ConstraintToken,
        },
        context::Context,
        visitor::ContextVisitor,
    },
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    std::collections::HashSet,
    syn::Ident,
};

struct Checks {
    has_token: bool,
    has_init: bool,
}

impl Checks {
    pub fn new() -> Self {
        Checks {
            has_token: false,
            has_init: false,
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
        self.has_init = true;
        Ok(())
    }

    fn visit_token(&mut self, _constraint: &ConstraintToken) -> Result<(), syn::Error> {
        self.has_token = true;
        Ok(())
    }
}

pub struct StateTokenGenerator<'a> {
    state_needed: HashSet<Ident>,
    current_account: Option<&'a Ident>,
}

impl<'a> StateTokenGenerator<'a> {
    pub fn analyze(context: &'a Context) -> Result<Self, syn::Error> {
        let mut state = StateTokenGenerator {
            state_needed: HashSet::new(),
            current_account: None,
        };

        for account in &context.accounts {
            state.current_account = Some(&account.name);

            let mut checks = Checks::new();
            checks.visit_account(account)?;

            state.visit_account(account)?;

            if checks.has_token && !checks.has_init && !state.state_needed.contains(&account.name) {
                state.state_needed.insert(account.name.to_owned());
            }
        }
        Ok(state)
    }

    pub fn get_token(&self, name: &'a Ident) -> Option<(TokenStream, Ident)> {
        if !self.state_needed.contains(name) {
            return None;
        }

        let var_name = format_ident!("{name}_state");
        let token = quote!(let #var_name = #name.data()?;);

        Some((token, var_name))
    }
}

// impl StagedGenerator for StateGenerator<'_> {
//     fn append(&mut self, result: &mut GeneratorResult) -> Result<(), syn::Error> {
//         for (name, has_bump) in self.state_needed.drain() {
//             let var_name = format_ident!("{name}_state");

//             let Some((has_init_if_needed, has_one)) = account_checks.get(&name) else {
//                 continue;
//             };

//             if has_bump && *has_init_if_needed && !*has_one {
//                 continue;
//             };

//             let token = quote!(let #var_name = #name.data()?;);
//             result.inside.extend(token);
//             result.drop_vars.push(var_name);
//         }

//         Ok(())
//     }
// }

impl ContextVisitor for StateTokenGenerator<'_> {
    fn visit_has_one(&mut self, _constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        let account_name = self.current_account.unwrap();
        self.state_needed.insert(account_name.clone());

        Ok(())
    }

    fn visit_bump(&mut self, constraint: &ConstraintBump) -> Result<(), syn::Error> {
        if let Some(c) = &constraint.0 {
            if let Some(name) = c.name() {
                self.state_needed.insert(name.clone());
            }
        }

        Ok(())
    }
}

use {
    crate::{context::ParsingContext, visitor::ContextVisitor},
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
    },
    typhoon_syn::{
        constraints::{
            ConstraintAddress, ConstraintAssert, ConstraintAssociatedToken, ConstraintBump,
            ConstraintHasOne, ConstraintInit, ConstraintInitIfNeeded, ConstraintPayer,
            ConstraintSeeded, ConstraintSeeds, ConstraintToken,
        },
        utils::{ContextExpr, SeedsExpr},
        InstructionAccount,
    },
};

pub struct DependencyLinker {
    dependencies: Vec<String>,
    has_init: bool,
    has_associated_token: bool,
}

impl DependencyLinker {
    fn new() -> Self {
        Self {
            dependencies: Vec::new(),
            has_init: false,
            has_associated_token: false,
        }
    }

    fn add_dependency(&mut self, ident: &impl ToString) {
        self.dependencies.push(ident.to_string());
    }

    fn add_dependencies_from_seeds(&mut self, seeds: &SeedsExpr) {
        let exprs: Box<dyn Iterator<Item = &syn::Expr>> = match seeds {
            SeedsExpr::Punctuated(punctuated) => Box::new(punctuated.iter()),
            SeedsExpr::Single(expr) => Box::new(std::iter::once(expr)),
        };
        for expr in exprs {
            for name in &ContextExpr::from(expr.clone()).names {
                self.add_dependency(name);
            }
        }
    }

    fn extract_dependencies(account: &InstructionAccount) -> Result<Vec<String>, syn::Error> {
        let mut linker = Self::new();
        linker.visit_account(account)?;

        if linker.has_init && linker.has_associated_token {
            linker.add_dependency(&"system_program");
            linker.add_dependency(&"token_program");
        }

        Ok(linker.dependencies)
    }
}

impl ContextVisitor for DependencyLinker {
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

    fn visit_payer(&mut self, constraint: &ConstraintPayer) -> Result<(), syn::Error> {
        self.add_dependency(&constraint.target);
        Ok(())
    }

    fn visit_bump(&mut self, constraint: &ConstraintBump) -> Result<(), syn::Error> {
        if let Some(ref bump) = constraint.0 {
            for name in &bump.names {
                self.add_dependency(&name);
            }
        }
        Ok(())
    }

    fn visit_token(&mut self, constraint: &ConstraintToken) -> Result<(), syn::Error> {
        if let ConstraintToken::Mint(ident) = constraint {
            self.add_dependency(ident)
        }
        Ok(())
    }

    fn visit_associated_token(
        &mut self,
        constraint: &ConstraintAssociatedToken,
    ) -> Result<(), syn::Error> {
        self.has_associated_token = true;
        match constraint {
            ConstraintAssociatedToken::Mint(ident) => self.add_dependency(ident),
            ConstraintAssociatedToken::Authority(ident) => self.add_dependency(ident),
        }
        Ok(())
    }

    fn visit_seeds(&mut self, constraint: &ConstraintSeeds) -> Result<(), syn::Error> {
        self.add_dependencies_from_seeds(&constraint.seeds);
        Ok(())
    }

    fn visit_seeded(&mut self, constraint: &ConstraintSeeded) -> Result<(), syn::Error> {
        if let Some(ref seeds) = constraint.0 {
            self.add_dependencies_from_seeds(seeds);
        }
        Ok(())
    }

    fn visit_has_one(&mut self, constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        self.add_dependency(&constraint.join_target);
        Ok(())
    }

    fn visit_address(&mut self, constraint: &ConstraintAddress) -> Result<(), syn::Error> {
        for name in &constraint.check.names {
            self.add_dependency(&name);
        }
        Ok(())
    }

    fn visit_assert(&mut self, constraint: &ConstraintAssert) -> Result<(), syn::Error> {
        for name in &constraint.assert.names {
            self.add_dependency(&name);
        }
        Ok(())
    }
}

/// Topologically sorts accounts so dependencies are ordered before their dependents.
///
/// Uses Kahn's algorithm with a min-heap for deterministic alphabetical tie-breaking
/// within the same dependency level. Accounts involved in dependency cycles are
/// appended at the end in alphabetical order.
pub fn sort_accounts(context: &mut ParsingContext) -> Result<(), syn::Error> {
    let n = context.accounts.len();
    if n <= 1 {
        return Ok(());
    }

    let account_name = |i: usize| context.accounts[i].name.to_string();

    let index_of: HashMap<String, usize> = (0..n).map(|i| (account_name(i), i)).collect();

    let dependency_indices: Vec<Vec<usize>> = context
        .accounts
        .iter()
        .map(|account| {
            let dep_names = DependencyLinker::extract_dependencies(account)?;
            Ok(dep_names
                .iter()
                .filter_map(|name| index_of.get(name).copied())
                .collect())
        })
        .collect::<Result<_, syn::Error>>()?;

    let mut dependency_count = vec![0usize; n];
    let mut dependents: Vec<Vec<usize>> = vec![vec![]; n];

    for (account, deps) in dependency_indices.iter().enumerate() {
        for &dep in deps {
            dependents[dep].push(account);
            dependency_count[account] += 1;
        }
    }

    let mut ready: BinaryHeap<Reverse<(String, usize)>> = dependency_count
        .iter()
        .enumerate()
        .filter(|(_, &count)| count == 0)
        .map(|(i, _)| Reverse((account_name(i), i)))
        .collect();

    let mut sorted = Vec::with_capacity(n);

    while let Some(Reverse((_, current))) = ready.pop() {
        sorted.push(current);

        for &dependent in &dependents[current] {
            dependency_count[dependent] -= 1;
            if dependency_count[dependent] == 0 {
                ready.push(Reverse((account_name(dependent), dependent)));
            }
        }
    }

    if sorted.len() < n {
        let visited: HashSet<usize> = sorted.iter().copied().collect();
        let mut remaining: Vec<usize> = (0..n).filter(|i| !visited.contains(i)).collect();
        remaining.sort_by_key(|&i| account_name(i));
        sorted.extend(remaining);
    }

    let reordered: Vec<InstructionAccount> = sorted
        .iter()
        .map(|&i| context.accounts[i].clone())
        .collect();
    context.accounts = reordered;

    Ok(())
}

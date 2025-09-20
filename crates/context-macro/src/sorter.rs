use {
    crate::{context::ParsingContext, visitor::ContextVisitor},
    std::collections::HashMap,
    typhoon_syn::{
        constraints::{
            ConstraintAssociatedToken, ConstraintBump, ConstraintHasOne, ConstraintPayer,
            ConstraintToken,
        },
        Account,
    },
};

pub struct DependencyLinker {
    dependencies: Vec<String>,
}

impl DependencyLinker {
    fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }

    fn add_dependency(&mut self, ident: &impl ToString) {
        self.dependencies.push(ident.to_string());
    }

    fn extract_dependencies(account: &Account) -> Result<Vec<String>, syn::Error> {
        let mut linker = Self::new();
        linker.visit_account(account)?;
        Ok(linker.dependencies)
    }
}

impl ContextVisitor for DependencyLinker {
    fn visit_payer(&mut self, constraint: &ConstraintPayer) -> Result<(), syn::Error> {
        self.add_dependency(&constraint.target);
        Ok(())
    }

    fn visit_bump(&mut self, constraint: &ConstraintBump) -> Result<(), syn::Error> {
        if let Some(ref bump) = constraint.0 {
            if let Some(name) = bump.name() {
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
        match constraint {
            ConstraintAssociatedToken::Mint(ident) => self.add_dependency(ident),
            ConstraintAssociatedToken::Authority(ident) => self.add_dependency(ident),
        }
        Ok(())
    }

    fn visit_has_one(&mut self, constraint: &ConstraintHasOne) -> Result<(), syn::Error> {
        self.add_dependency(&constraint.join_target);
        Ok(())
    }
}

pub fn sort_accounts(context: &mut ParsingContext) -> Result<(), syn::Error> {
    let account_dependencies = context
        .accounts
        .iter()
        .map(|account| {
            let dependencies = DependencyLinker::extract_dependencies(account)?;
            Ok((account, dependencies))
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    let name_to_index: HashMap<String, usize> = account_dependencies
        .iter()
        .enumerate()
        .map(|(i, (account, _))| (account.name.to_string(), i))
        .collect();

    let mut in_degree = vec![0; account_dependencies.len()];
    let mut adj_list: Vec<Vec<usize>> = vec![vec![]; account_dependencies.len()];

    for (i, (_, dependencies)) in account_dependencies.iter().enumerate() {
        for dep_name in dependencies {
            if let Some(&dep_index) = name_to_index.get(dep_name) {
                // dep_index should come before i
                adj_list[dep_index].push(i);
                in_degree[i] += 1;
            }
        }
    }

    let mut queue = Vec::new();
    let mut result = Vec::new();

    for (i, &degree) in in_degree.iter().enumerate() {
        if degree == 0 {
            queue.push(i);
        }
    }

    queue.sort_by(|&a, &b| {
        account_dependencies[a]
            .0
            .name
            .cmp(&account_dependencies[b].0.name)
    });

    while let Some(current) = queue.pop() {
        result.push(current);

        let mut neighbors = adj_list[current].clone();
        neighbors.sort_by(|&a, &b| {
            account_dependencies[a]
                .0
                .name
                .cmp(&account_dependencies[b].0.name)
        });

        for &neighbor in &neighbors {
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                let pos = queue
                    .binary_search_by(|&probe| {
                        account_dependencies[probe]
                            .0
                            .name
                            .cmp(&account_dependencies[neighbor].0.name)
                    })
                    .unwrap_or_else(|pos| pos);
                queue.insert(pos, neighbor);
            }
        }
    }

    if result.len() != account_dependencies.len() {
        let mut remaining: Vec<usize> = (0..account_dependencies.len())
            .filter(|&i| !result.contains(&i))
            .collect();

        remaining.sort_by(|&a, &b| {
            account_dependencies[a]
                .0
                .name
                .cmp(&account_dependencies[b].0.name)
        });

        result.extend(remaining);
    }

    context.accounts = result
        .into_iter()
        .map(|i| account_dependencies[i].0.clone())
        .collect();

    Ok(())
}

use {
    crate::{context::ParsingContext, visitor::ContextVisitor},
    std::collections::{BinaryHeap, HashMap, HashSet},
    typhoon_syn::{
        constraints::{
            ConstraintAddress, ConstraintAssert, ConstraintAssociatedToken, ConstraintBump,
            ConstraintHasOne, ConstraintPayer, ConstraintToken,
        },
        InstructionAccount,
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

    fn extract_dependencies(account: &InstructionAccount) -> Result<Vec<String>, syn::Error> {
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

// Wrapper for using indices with BinaryHeap (min-heap by name)
#[derive(Eq, PartialEq)]
struct HeapNode {
    index: usize,
    name: String,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse for min-heap behavior
        other.name.cmp(&self.name)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

    // Sort adjacency lists once upfront
    for neighbors in &mut adj_list {
        neighbors.sort_unstable_by(|&a, &b| {
            account_dependencies[a]
                .0
                .name
                .cmp(&account_dependencies[b].0.name)
        });
    }

    let mut heap = BinaryHeap::new();
    let mut result = Vec::with_capacity(account_dependencies.len());

    for (i, &degree) in in_degree.iter().enumerate() {
        if degree == 0 {
            heap.push(HeapNode {
                index: i,
                name: account_dependencies[i].0.name.to_string(),
            });
        }
    }

    while let Some(HeapNode { index: current, .. }) = heap.pop() {
        result.push(current);

        for &neighbor in &adj_list[current] {
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                heap.push(HeapNode {
                    index: neighbor,
                    name: account_dependencies[neighbor].0.name.to_string(),
                });
            }
        }
    }

    if result.len() != account_dependencies.len() {
        let result_set: HashSet<usize> = result.iter().copied().collect();
        let mut remaining: Vec<usize> = (0..account_dependencies.len())
            .filter(|i| !result_set.contains(i))
            .collect();

        remaining.sort_unstable_by(|&a, &b| {
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

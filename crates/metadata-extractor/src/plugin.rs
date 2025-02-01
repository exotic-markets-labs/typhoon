use {
    crate::{
        helpers::ItemHelper,
        visitors::{CacheByImplsVisitor, CacheInstructionIdents, SetProgramIdVisitor},
    },
    codama::{
        AccountNode, CodamaResult, CombineModulesVisitor, CombineTypesVisitor, FilterItemsVisitor,
        KorokMut, KorokTrait, KorokVisitor, NestedTypeNode, Node, SetProgramMetadataVisitor,
        StructTypeNode, UniformVisitor,
    },
    codama_korok_plugins::KorokPlugin,
    codama_korok_visitors::{
        ComposeVisitor, KorokVisitable, SetBorshTypesVisitor, SetLinkTypesVisitor,
    },
    std::collections::HashSet,
};

pub struct TyphoonPlugin;

impl KorokPlugin for TyphoonPlugin {
    fn run(
        &self,
        visitable: &mut dyn KorokVisitable,
        next: &dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()>,
    ) -> CodamaResult<()> {
        next(visitable)?;

        let mut cache_accounts = HashSet::new();
        let mut cache_instructions = HashSet::new();

        {
            let mut first_visitor = ComposeVisitor::new()
                .add(CacheByImplsVisitor::new(&["Owner"], &mut cache_accounts))
                .add(CacheInstructionIdents::new(&mut cache_instructions));
            visitable.accept(&mut first_visitor)?;
        }

        let cache_accounts: Vec<String> = cache_accounts.into_iter().collect();
        let cache_accounts_ref = &cache_accounts;
        let _cache_instructions: Vec<String> = cache_instructions.into_iter().collect();

        println!("{:?}", _cache_instructions);

        let mut default_visitor = ComposeVisitor::new()
            .add(FilterItemsVisitor::new(
                move |item| {
                    item.has_name_in_cache(cache_accounts_ref)
                        || item.has_attribute("account")
                        || item.has_attribute("context")
                },
                ComposeVisitor::new()
                    .add(SetBorshTypesVisitor::new())
                    .add(SetLinkTypesVisitor::new())
                    .add(CombineTypesVisitor::new()),
            ))
            .add(FilterItemsVisitor::new(
                move |item| {
                    item.has_name_in_cache(cache_accounts_ref) || item.has_attribute("account")
                },
                UniformVisitor::new(|mut k, visitor| {
                    visitor.visit_children(&mut k)?;
                    apply_account(k);
                    Ok(())
                }),
            ))
            .add(SetProgramIdVisitor::new())
            .add(SetProgramMetadataVisitor::new())
            .add(CombineModulesVisitor::new());

        visitable.accept(&mut default_visitor)?;

        Ok(())
    }
}

fn apply_account(mut korok: KorokMut) {
    let Some(Node::DefinedType(ref def_ty)) = korok.node() else {
        return;
    };

    let Ok(data) = NestedTypeNode::<StructTypeNode>::try_from(def_ty.r#type.clone()) else {
        return;
    };

    let account = AccountNode {
        name: def_ty.name.clone(),
        docs: def_ty.docs.clone(),
        size: None,
        pda: None,
        discriminators: Vec::new(),
        data,
    };

    korok.set_node(Some(Node::Account(account)));
}

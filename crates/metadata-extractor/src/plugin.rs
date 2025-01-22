use {
    crate::{
        helpers::ItemHelper,
        visitors::{CacheByImplsVisitor, SetProgramIdVisitor},
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
};

pub struct TyphoonPlugin;

impl KorokPlugin for TyphoonPlugin {
    fn run(
        &self,
        visitable: &mut dyn KorokVisitable,
        next: &dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()>,
    ) -> CodamaResult<()> {
        next(visitable)?;

        let mut cache = CacheByImplsVisitor::new(&["Owner"]);
        visitable.accept(&mut cache)?;
        let account_cache = cache.get_cache();

        let cache_slice = account_cache.as_slice();
        let mut default_visitor = ComposeVisitor::new()
            .add(FilterItemsVisitor::new(
                move |item| {
                    item.has_name_in_cache(cache_slice)
                        || item.has_attribute("account")
                        || item.has_attribute("context")
                },
                ComposeVisitor::new()
                    .add(SetBorshTypesVisitor::new())
                    .add(SetLinkTypesVisitor::new())
                    .add(CombineTypesVisitor::new()),
            ))
            .add(FilterItemsVisitor::new(
                move |item| item.has_name_in_cache(cache_slice) || item.has_attribute("account"),
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

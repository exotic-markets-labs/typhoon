use {
    crate::{
        helpers::ItemHelper,
        visitors::{
            CacheByImplsVisitor, CacheInstructionIdents, ContextVisitor, DoubleVisitor,
            InstructionVisitor, SetDefinedTypesVisitor, SetProgramIdVisitor,
        },
    },
    codama::{
        AccountNode, CodamaResult, CombineModulesVisitor, CombineTypesVisitor, ComposeVisitor,
        FilterItemsVisitor, KorokMut, KorokTrait, KorokVisitor, NestedTypeNode, Node,
        SetBorshTypesVisitor, SetLinkTypesVisitor, SetProgramMetadataVisitor, StructTypeNode,
        UniformVisitor,
    },
    codama_korok_plugins::KorokPlugin,
    codama_korok_visitors::KorokVisitable,
    std::{
        collections::{HashMap, HashSet},
        rc::Rc,
    },
    typhoon_discriminator::DiscriminatorBuilder,
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
        let mut cache_instructions = Rc::new(HashMap::new());

        {
            let mut first_visitor = ComposeVisitor::new()
                .with(CacheByImplsVisitor::new(&["Owner"], &mut cache_accounts))
                .with(CacheInstructionIdents::new(
                    Rc::get_mut(&mut cache_instructions).unwrap(),
                ));
            visitable.accept(&mut first_visitor)?;
        }

        let cache_instructions_cloned = cache_instructions.clone();
        let mut default_visitor = ComposeVisitor::new()
            .with(FilterItemsVisitor::new(
                move |item| item.has_attribute("account"),
                ComposeVisitor::new()
                    .with(SetBorshTypesVisitor::new())
                    .with(SetLinkTypesVisitor::new())
                    .with(CombineTypesVisitor::new())
                    .with(UniformVisitor::new(|mut k, visitor| {
                        visitor.visit_children(&mut k)?;
                        apply_account(k);
                        Ok(())
                    })),
            ))
            .with(FilterItemsVisitor::new(
                move |item| {
                    item.name()
                        .map(|n| cache_instructions_cloned.contains_key(&n))
                        .unwrap_or_default()
                        || item.has_attribute("context")
                },
                ComposeVisitor::new()
                    .with(ContextVisitor::new())
                    .with(InstructionVisitor::new(&cache_instructions)),
            ))
            .with(DoubleVisitor::new(SetDefinedTypesVisitor::new()))
            .with(SetProgramIdVisitor::new())
            .with(SetProgramMetadataVisitor::new())
            .with(CombineModulesVisitor::new());

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

    let _calculated_dis = DiscriminatorBuilder::new(def_ty.name.as_str()).build();

    // ConstantDiscriminatorNode::new(ConstantValueNode::bytes(), 0);

    let account = AccountNode {
        name: def_ty.name.clone(),
        docs: def_ty.docs.clone(),
        size: None,
        pda: None,
        discriminators: Vec::from([]),
        data,
    };
    korok.set_node(Some(Node::Account(account)));
}

use {
    crate::{helpers::AttributesHelper, visitors::SetProgramIdVisitor},
    codama::{CombineModulesVisitor, CombineTypesVisitor, SetProgramMetadataVisitor},
    codama_korok_plugins::KorokPlugin,
    codama_korok_visitors::{
        ComposeVisitor, FilterItemsVisitor, KorokVisitable, SetBorshTypesVisitor,
        SetLinkTypesVisitor,
    },
    codama_koroks::KorokTrait,
};

pub struct TyphoonPlugin;

impl KorokPlugin for TyphoonPlugin {
    fn run(&self, visitable: &mut dyn KorokVisitable, next: &dyn Fn(&mut dyn KorokVisitable)) {
        next(visitable);

        let compose_visitor = || {
            ComposeVisitor::new()
                .add(SetBorshTypesVisitor::new())
                .add(SetLinkTypesVisitor::new())
        };

        let mut default_visitor = ComposeVisitor::new()
            .add(FilterItemsVisitor::new(
                |item| item.attributes().unwrap().has_attribute("account"),
                compose_visitor(),
            ))
            .add(SetProgramIdVisitor::new())
            .add(SetProgramMetadataVisitor::new())
            .add(FilterItemsVisitor::new(
                |item| item.attributes().unwrap().has_attribute("account"),
                CombineTypesVisitor::new(),
            ))
            .add(CombineModulesVisitor::new());

        visitable.accept(&mut default_visitor);
    }
}

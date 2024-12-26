use {
    crate::helpers::AttributesHelper,
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
        visitable.accept(&mut get_default_visitor());
    }
}

pub fn get_default_visitor<'a>() -> ComposeVisitor<'a> {
    ComposeVisitor::new()
        .add(FilterItemsVisitor::new(
            |item| item.attributes().unwrap().has_attribute("account"),
            ComposeVisitor::new()
                .add(SetBorshTypesVisitor::new())
                .add(SetLinkTypesVisitor::new()),
        ))
        .add(SetProgramMetadataVisitor::new())
        // .add(ApplyCodamaAttributesVisitor::new())
        .add(FilterItemsVisitor::new(
            |item| item.attributes().unwrap().has_any_codama_derive(),
            CombineTypesVisitor::new(),
        ))
        .add(CombineModulesVisitor::new())
}

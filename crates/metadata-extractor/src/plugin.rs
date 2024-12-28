use {
    crate::{
        helpers::ItemHelper,
        visitors::{CacheByImplsVisitor, FilterItemsVisitor, SetProgramIdVisitor},
    },
    codama::{CombineModulesVisitor, SetProgramMetadataVisitor},
    codama_korok_plugins::KorokPlugin,
    codama_korok_visitors::{
        ComposeVisitor, KorokVisitable, SetBorshTypesVisitor, SetLinkTypesVisitor,
    },
};

pub struct TyphoonPlugin;

impl KorokPlugin for TyphoonPlugin {
    fn run(&self, visitable: &mut dyn KorokVisitable, next: &dyn Fn(&mut dyn KorokVisitable)) {
        next(visitable);

        let mut cache = CacheByImplsVisitor::new(&["Owner"]);
        visitable.accept(&mut cache);

        let struct_cache = cache.get_cache();

        let compose_visitor = || {
            ComposeVisitor::new()
                .add(SetBorshTypesVisitor::new())
                .add(SetLinkTypesVisitor::new())
        };

        let cache_slice = struct_cache.as_slice();
        let mut default_visitor = ComposeVisitor::new()
            .add(FilterItemsVisitor::new(
                move |item| {
                    item.has_name_in_cache(cache_slice)
                        || item.has_attribute("account")
                        || item.has_attribute("context")
                },
                compose_visitor(),
            ))
            .add(SetProgramIdVisitor::new())
            .add(SetProgramMetadataVisitor::new())
            .add(FilterItemsVisitor::new(
                move |item| {
                    item.has_name_in_cache(cache_slice)
                        || item.has_attribute("account")
                        || item.has_attribute("context")
                },
                compose_visitor(),
            ))
            .add(CombineModulesVisitor::new());

        visitable.accept(&mut default_visitor);
    }
}

use {
    codama::{CodamaResult, KorokVisitor},
    codama_syn_helpers::extensions::{PathExtension, TypeExtension},
    std::collections::HashSet,
    syn::Item,
};

pub struct CacheByImplsVisitor<'a> {
    traits: &'a [&'static str],
    cache: &'a mut HashSet<String>,
}

impl<'a> CacheByImplsVisitor<'a> {
    pub fn new(traits: &'a [&'static str], cache: &'a mut HashSet<String>) -> Self {
        CacheByImplsVisitor { traits, cache }
    }
}

impl KorokVisitor for CacheByImplsVisitor<'_> {
    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;

        let Item::Impl(impl_item) = korok.ast else {
            return Ok(());
        };

        let Some((_, trait_path, _)) = &impl_item.trait_ else {
            return Ok(());
        };

        if !self
            .traits
            .iter()
            .any(|trait_name| trait_path.last().ident == trait_name)
        {
            return Ok(());
        }

        if let Ok(impl_path) = impl_item.self_ty.as_path() {
            self.cache.insert(impl_path.last_str());
        }

        Ok(())
    }
}

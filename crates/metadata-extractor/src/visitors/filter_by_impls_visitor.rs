use std::collections::HashSet;

use codama_korok_visitors::KorokVisitor;
use codama_syn_helpers::extensions::{PathExtension, TypeExtension};
use syn::Item;

#[derive(Debug)]
pub struct FilterByImplsVisitor<'a> {
    pub traits: &'a [&'static str],
    pub struct_cache: HashSet<String>,
}

impl<'a> FilterByImplsVisitor<'a> {
    pub fn new(traits: &'a [&'static str]) -> Self {
        FilterByImplsVisitor {
            traits,
            struct_cache: HashSet::new(),
        }
    }
}

impl KorokVisitor for FilterByImplsVisitor<'_> {
    fn visit_unsupported_item(&mut self, korok: &mut codama_koroks::UnsupportedItemKorok) {
        self.visit_children(korok);

        let Item::Impl(impl_item) = korok.ast else {
            return;
        };

        let Some((_, trait_path, _)) = &impl_item.trait_ else {
            return;
        };

        if !self
            .traits
            .iter()
            .any(|trait_name| trait_path.last().ident == trait_name)
        {
            return;
        }

        if let Ok(impl_path) = impl_item.self_ty.as_path() {
            self.struct_cache.insert(impl_path.last_str());
        }
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) {
        if korok.node.is_some() {
            return;
        }

        if self.struct_cache.contains(&korok.ast.ident.to_string()) {}
    }
}

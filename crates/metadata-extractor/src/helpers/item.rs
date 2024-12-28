use codama::{ItemKorok, KorokTrait};

use super::AttributesHelper;

pub trait ItemHelper {
    fn has_attribute(&self, last: &str) -> bool;
    fn has_name_in_cache(&self, cache: &[String]) -> bool;
}

impl ItemHelper for ItemKorok<'_> {
    fn has_attribute(&self, last: &str) -> bool {
        self.attributes()
            .map(|attrs| attrs.has_attribute(last))
            .unwrap_or_default()
    }

    fn has_name_in_cache(&self, cache: &[String]) -> bool {
        match self {
            ItemKorok::Struct(struct_korok) => cache.iter().any(|el| struct_korok.ast.ident == el),
            _ => false,
        }
    }
}

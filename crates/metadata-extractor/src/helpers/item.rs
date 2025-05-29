use {
    super::AttributesHelper,
    codama::{ItemKorok, KorokTrait},
    syn::Item,
};

pub trait ItemHelper {
    fn has_attribute(&self, last: &str) -> bool;
    fn name(&self) -> Option<String>;
}

impl ItemHelper for ItemKorok<'_> {
    fn has_attribute(&self, last: &str) -> bool {
        self.attributes()
            .map(|attrs| attrs.has_attribute(last))
            .unwrap_or_default()
    }

    fn name(&self) -> Option<String> {
        match self {
            ItemKorok::Struct(struct_korok) => Some(struct_korok.ast.ident.to_string()),
            ItemKorok::Unsupported(unsupported_korok) => match unsupported_korok.ast {
                Item::Fn(item_fn) => Some(item_fn.sig.ident.to_string()),
                _ => None,
            },
            ItemKorok::Enum(enum_korok) => Some(enum_korok.ast.ident.to_string()),
            _ => None,
        }
    }
}

use codama_attributes::Attributes;
use codama_syn_helpers::extensions::PathExtension;

pub trait AttributesHelper {
    fn has_attribute(&self, last: &str) -> bool;
}

impl AttributesHelper for Attributes<'_> {
    fn has_attribute(&self, last: &str) -> bool {
        self.iter().any(|attr| match attr {
            codama_attributes::Attribute::Unsupported(a) => a.ast.path().last_str() == last,
            _ => false,
        })
    }
}

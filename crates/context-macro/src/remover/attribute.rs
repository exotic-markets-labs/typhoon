use syn::visit_mut::VisitMut;

pub struct AttributeRemover(&'static str);

impl AttributeRemover {
    pub fn new(name: &'static str) -> Self {
        AttributeRemover(name)
    }
}

impl VisitMut for AttributeRemover {
    fn visit_attributes_mut(&mut self, i: &mut Vec<syn::Attribute>) {
        i.retain(|el| !el.path().is_ident(self.0));
    }
}

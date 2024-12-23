use syn::{visit_mut::VisitMut, Field};

pub struct FieldInjector(Field);

impl FieldInjector {
    pub fn new(field: Field) -> Self {
        FieldInjector(field)
    }
}

impl VisitMut for FieldInjector {
    fn visit_fields_named_mut(&mut self, i: &mut syn::FieldsNamed) {
        i.named.push(self.0.to_owned());
    }
}

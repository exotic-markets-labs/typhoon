use syn::fold::Fold;

pub struct AttributeRemover(Vec<&'static str>);

impl AttributeRemover {
    pub fn new() -> Self {
        AttributeRemover(Vec::new())
    }

    pub fn with_attribute(mut self, attr: &'static str) -> Self {
        self.0.push(attr);
        self
    }
}

impl Fold for AttributeRemover {
    fn fold_attributes(&mut self, mut i: Vec<syn::Attribute>) -> Vec<syn::Attribute> {
        i.retain(|el| {
            !el.path()
                .get_ident()
                .map(|p| self.0.contains(&p.to_string().as_str()))
                .unwrap_or_default()
        });
        i
    }
}

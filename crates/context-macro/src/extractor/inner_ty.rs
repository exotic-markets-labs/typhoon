use syn::visit::Visit;

pub struct InnerTyExtractor {
    pub ty: Option<String>,
}

impl InnerTyExtractor {
    pub fn new() -> Self {
        Self { ty: None }
    }
}

impl Visit<'_> for InnerTyExtractor {
    fn visit_path(&mut self, i: &'_ syn::Path) {
        if let Some(seg) = i.segments.last() {
            self.ty = Some(seg.ident.to_string());
            self.visit_path_segment(seg);
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, syn::parse_quote};

    #[test]
    fn test_inner_ty_extractor() {
        // Test with a simple path
        let mut extractor = InnerTyExtractor::new();
        let path: syn::Path = parse_quote!(Program<System>);
        extractor.visit_path(&path);
        assert_eq!(extractor.ty, Some("System".to_string()));

        let path: syn::Path = parse_quote!(Mut<Account<'info, Random>>);
        extractor.visit_path(&path);
        assert_eq!(extractor.ty, Some("Random".to_string()));

        let field: syn::Field = parse_quote!(pub random2: Account<'info, Random2>);
        extractor.visit_field(&field);
        assert_eq!(extractor.ty, Some("Random2".to_string()));
    }
}

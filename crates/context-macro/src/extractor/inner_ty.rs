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
    fn visit_field(&mut self, field: &'_ syn::Field) {
        self.visit_type(&field.ty);
    }

    fn visit_path(&mut self, i: &'_ syn::Path) {
        if let Some(segment) = i.segments.last() {
            if matches!(segment.arguments, syn::PathArguments::AngleBracketed(_)) {
                self.visit_path_segment(segment);
            }

            if self.ty.is_none() {
                self.ty = Some(segment.ident.to_string());
            }
        }
    }

    fn visit_path_segment(&mut self, segment: &'_ syn::PathSegment) {
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            for arg in &args.args {
                if let syn::GenericArgument::Type(ty) = arg {
                    self.visit_type(ty);

                    if self.ty.is_some() {
                        return;
                    }
                }
            }
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

        let mut extractor = InnerTyExtractor::new();
        let path: syn::Path = parse_quote!(Mut<Account<'info, Random>>);
        extractor.visit_path(&path);
        assert_eq!(extractor.ty, Some("Random".to_string()));

        let mut extractor = InnerTyExtractor::new();
        let path: syn::Path = parse_quote!(Mut<SystemAccount, NoCheck>);
        extractor.visit_path(&path);
        assert_eq!(extractor.ty, Some("SystemAccount".to_string()));

        let mut extractor = InnerTyExtractor::new();
        let field: syn::Field = parse_quote!(pub random2: Account<'info, Random2>);
        extractor.visit_field(&field);
        assert_eq!(extractor.ty, Some("Random2".to_string()));

        let mut extractor = InnerTyExtractor::new();
        let field: syn::Field = parse_quote!(pub random2: Signer);
        extractor.visit_field(&field);
        assert_eq!(extractor.ty, Some("Signer".to_string()));

        let mut extractor = InnerTyExtractor::new();
        let field: syn::Field = parse_quote! {
            #[constraint(
                init_if_needed,
                payer = payer,
                associated_token::mint = mint,
                associated_token::authority = owner
            )]
            pub random2: Mut<Account<TokenAccount>>
        };
        extractor.visit_field(&field);
        assert_eq!(extractor.ty, Some("TokenAccount".to_string()));
    }
}

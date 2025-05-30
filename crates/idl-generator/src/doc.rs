use syn::{Attribute, Expr, ExprLit, Lit};

#[derive(Default, Debug)]
pub struct Docs(Vec<String>);

impl From<&[Attribute]> for Docs {
    fn from(value: &[Attribute]) -> Self {
        let docs = value
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .filter_map(|attr| {
                if let syn::Meta::NameValue(v) = &attr.meta {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(str_lit),
                        ..
                    }) = &v.value
                    {
                        return Some(str_lit.value().trim().to_string());
                    }
                }
                None
            })
            .collect();

        Docs(docs)
    }
}

impl Docs {
    pub fn into_vec(self) -> Vec<String> {
        self.0
    }
}

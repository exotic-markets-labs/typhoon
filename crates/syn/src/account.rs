use {
    crate::constraints::{Constraint, Constraints},
    proc_macro2::Span,
    syn::{
        parse_quote,
        visit::{visit_path_segment, Visit},
        Field, Ident, PathSegment,
    },
};

#[derive(Default, Clone)]
pub struct AccountMeta {
    pub is_signer: bool,
    pub is_mutable: bool,
    pub is_optional: bool,
}

#[derive(Clone)]
pub struct Account {
    pub name: Ident,
    pub constraints: Constraints,
    pub meta: AccountMeta,
    ty: Option<PathSegment>,
    pub inner_ty: Ident,
}

impl Account {
    pub fn is_init_signer(&self) -> bool {
        let has_init = self
            .constraints
            .0
            .iter()
            .any(|c| matches!(c, Constraint::Init(_) | Constraint::InitIfNeeded(_)));
        let is_pda = self
            .constraints
            .0
            .iter()
            .any(|c| matches!(c, Constraint::Bump(_) | Constraint::AssociatedToken(_)));

        has_init && !is_pda
    }
}

impl Account {
    pub fn get_ty(&self) -> PathSegment {
        let inner_ty = &self.inner_ty;
        self.ty.clone().unwrap_or(parse_quote!(#inner_ty<'info>))
    }
}

impl TryFrom<&Field> for Account {
    type Error = syn::Error;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        let mut acc = Account {
            name: value
                .ident
                .clone()
                .ok_or(syn::Error::new_spanned(value, "The field need to be named"))?,
            constraints: Constraints::try_from(value.attrs.as_slice())?,
            meta: AccountMeta::default(),
            ty: None,
            inner_ty: Ident::new("UncheckedAccount", Span::call_site()),
        };

        acc.visit_type(&value.ty);
        Ok(acc)
    }
}

impl Visit<'_> for Account {
    fn visit_path_segment(&mut self, i: &syn::PathSegment) {
        let ty = i.ident.to_string();

        if ty == "Option" {
            self.meta.is_optional = true;
            visit_path_segment(self, i);
        } else if ty.starts_with("Mut") {
            self.meta.is_mutable = true;
            if self.ty.is_none() {
                self.ty = Some(i.clone());
            }
            visit_path_segment(self, i);
        } else if ty.starts_with("Signer") {
            self.meta.is_signer = true;
            if self.ty.is_none() {
                self.ty = Some(i.clone());
            }
            visit_path_segment(self, i);
        } else {
            if self.ty.is_none() {
                self.ty = Some(i.clone());
            }
            self.inner_ty = i.ident.clone();
            visit_path_segment(self, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, syn::parse_quote};

    #[test]
    fn test_instruction_parse_with_context() {
        let field: syn::Field = parse_quote!(pub random2: Option<Mut<Account<Random2>>>);
        let account = Account::try_from(&field).unwrap();
        assert_eq!(account.inner_ty, "Random2");
        assert!(account.meta.is_mutable);
        assert!(account.meta.is_optional);
        assert!(!account.meta.is_signer);

        let field: syn::Field = parse_quote!(pub random2: SignerNoCheck<'info, Account<Random2>>);
        let account = Account::try_from(&field).unwrap();
        assert_eq!(account.inner_ty, "Random2");
        assert!(!account.meta.is_mutable);
        assert!(!account.meta.is_optional);
        assert!(account.meta.is_signer);

        let field: syn::Field = parse_quote!(pub random2: InterfaceAccount<TokenAccount>);
        let account = Account::try_from(&field).unwrap();
        assert_eq!(account.inner_ty, "TokenAccount");
        assert!(!account.meta.is_mutable);
        assert!(!account.meta.is_optional);
        assert!(!account.meta.is_signer);

        let field: syn::Field = parse_quote!(pub random2: UncheckedAccount);
        let account = Account::try_from(&field).unwrap();
        assert_eq!(account.inner_ty, "UncheckedAccount");
        assert!(!account.meta.is_mutable);
        assert!(!account.meta.is_optional);
        assert!(!account.meta.is_signer);
    }
}

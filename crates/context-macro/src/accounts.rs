use {
    crate::{
        constraints::{Constraints, CONSTRAINT_IDENT_STR},
        remover::AttributeRemover,
    },
    proc_macro2::Span,
    syn::{spanned::Spanned, visit_mut::VisitMut, Field, Ident, PathSegment, Type, TypePath},
};

#[derive(Clone)]
pub struct Account {
    pub(crate) name: Ident,
    pub(crate) constraints: Constraints,
    pub(crate) ty: PathSegment,
}

impl TryFrom<&mut Field> for Account {
    type Error = syn::Error;

    fn try_from(value: &mut Field) -> Result<Self, Self::Error> {
        let constraints = Constraints::try_from(&value.attrs)?;
        AttributeRemover::new(CONSTRAINT_IDENT_STR).visit_field_mut(value);

        let segment = match &value.ty {
            Type::Path(TypePath { path, .. }) => path.segments.last(),
            _ => None,
        }
        .ok_or_else(|| syn::Error::new(value.span(), "Invalid type for the account"))?;

        let name = value
            .ident
            .clone()
            .unwrap_or(Ident::new("random", Span::call_site())); //TODO unit type

        Ok(Account {
            name,
            constraints,
            ty: segment.clone(),
        })
    }
}

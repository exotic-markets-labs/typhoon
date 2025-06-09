use {
    crate::{extractor::InnerTyExtractor, remover::AttributeRemover},
    proc_macro2::Span,
    syn::{
        spanned::Spanned, visit::Visit, visit_mut::VisitMut, Field, Ident, PathSegment, Type,
        TypePath,
    },
    typhoon_syn::constraints::{Constraints, CONSTRAINT_IDENT_STR},
};

#[derive(Clone)]
pub struct Account {
    pub name: Ident,
    pub constraints: Constraints,
    pub ty: PathSegment,
    pub is_optional: bool,
    pub inner_ty: String,
}

impl TryFrom<&mut Field> for Account {
    type Error = syn::Error;

    fn try_from(value: &mut Field) -> Result<Self, Self::Error> {
        let mut inner_ty_extractor = InnerTyExtractor::new();
        inner_ty_extractor.visit_field(value);
        let inner_ty = inner_ty_extractor
            .ty
            .ok_or(syn::Error::new(value.span(), "Cannot find the inner type."))?;

        let constraints = Constraints::try_from(value.attrs.as_slice())?;
        AttributeRemover::new(CONSTRAINT_IDENT_STR).visit_field_mut(value);

        let segment = match &value.ty {
            Type::Path(TypePath { path, .. }) => path.segments.last(),
            _ => None,
        }
        .ok_or_else(|| syn::Error::new(value.span(), "Invalid type for the account."))?;

        let name = value
            .ident
            .clone()
            .unwrap_or(Ident::new("random", Span::call_site())); //TODO unit type

        let (ty, is_optional) = if segment.ident == "Option" {
            let inner_segment = get_inner(segment).ok_or_else(|| {
                syn::Error::new(segment.span(), "Invalid Option type for the account.")
            })?;
            (inner_segment, true)
        } else {
            (segment, false)
        };

        Ok(Account {
            name,
            constraints,
            ty: ty.clone(),
            is_optional,
            inner_ty,
        })
    }
}

fn get_inner(seg: &PathSegment) -> Option<&PathSegment> {
    match &seg.arguments {
        syn::PathArguments::AngleBracketed(args) => match args.args.first()? {
            syn::GenericArgument::Type(Type::Path(p)) => Some(p.path.segments.last()?),
            _ => None,
        },
        _ => None,
    }
}

use {
    crate::{extractor::InnerTyExtractor, remover::AttributeRemover},
    pinocchio::cpi::MAX_CPI_ACCOUNTS,
    proc_macro2::Span,
    syn::{
        spanned::Spanned, visit::Visit, visit_mut::VisitMut, Expr, ExprLit, ExprPath, Field, Ident,
        Lit, PathSegment, Type, TypeArray, TypePath,
    },
    typhoon_syn::constraints::{Constraints, CONSTRAINT_IDENT_STR},
};

const MAX_ARRAY_SIZE: usize = MAX_CPI_ACCOUNTS; // using the MAX_CPI_ACCOUNTS from pinocchio to limit the array size

#[derive(Clone)]
pub struct Account {
    pub name: Ident,
    pub constraints: Constraints,
    pub ty: PathSegment,
    pub is_optional: bool,
    pub inner_ty: String,
    pub is_array: bool,
    pub array_size: Option<usize>,
    pub const_generic: Option<Ident>,
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

        let name = value
            .ident
            .clone()
            .unwrap_or(Ident::new("random", Span::call_site())); //TODO unit type

        // Handle both regular types and array types
        match &value.ty {
            Type::Path(TypePath { path, .. }) => {
                let segment = path.segments.last().ok_or_else(|| {
                    syn::Error::new(value.span(), "Invalid type for the account.")
                })?;

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
                    is_array: false,
                    array_size: None,
                    const_generic: None,
                })
            }
            Type::Array(TypeArray { elem, len, .. }) => {
                // Extract the element type from the array
                let elem_path = match elem.as_ref() {
                    Type::Path(TypePath { path, .. }) => path.segments.last(),
                    _ => None,
                }
                .ok_or_else(|| {
                    syn::Error::new(elem.span(), "Invalid array element type for account.")
                })?;

                // Extract array size from the length expression
                let (array_size, const_generic) = extract_array_size(len).ok_or_else(|| {
                    syn::Error::new(
                        len.span(),
                        "Invalid array size. Must be a constant integer between 1 and 64, \
                         or a const generic parameter. Zero-sized arrays and arrays larger \
                         than 64 elements are not allowed for security and performance reasons.",
                    )
                })?;

                let (ty, is_optional) = if elem_path.ident == "Option" {
                    let inner_segment = get_inner(elem_path).ok_or_else(|| {
                        syn::Error::new(elem_path.span(), "Invalid Option type for the account.")
                    })?;
                    (inner_segment, true)
                } else {
                    (elem_path, false)
                };

                Ok(Account {
                    name,
                    constraints,
                    ty: ty.clone(),
                    is_optional,
                    inner_ty,
                    is_array: true,
                    array_size,
                    const_generic,
                })
            }
            _ => Err(syn::Error::new(
                value.span(),
                "Invalid type for the account. Expected Path or Array.",
            )),
        }
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


fn extract_array_size(len: &Expr) -> Option<(Option<usize>, Option<Ident>)> {
    match len {
        Expr::Lit(ExprLit {
            lit: Lit::Int(int), ..
        }) => {
            match int.base10_parse::<usize>() {
                Ok(size) => {
                    if size == 0 {
                        return None;
                    }
                    // Security: Prevent excessively large arrays that could cause
                    // compilation issues, stack overflow, or resource exhaustion
                    if size > MAX_ARRAY_SIZE {
                        return None;
                    }
                    Some((Some(size), None))
                }
                Err(_) => None, // Handle integer overflow/underflow
            }
        }
        Expr::Path(ExprPath { path, .. }) => {
            // Handle const generic parameters like 'N'
            if let Some(segment) = path.segments.last() {
                let const_name = segment.ident.clone();
                // For const generics, we don't validate the size at macro expansion time
                // Validation must happen where the const parameter is defined:
                // const N: usize = 5; // Must be: 1 <= N <= MAX_CPI_ACCOUNTS
                // This ensures security and performance constraints are maintained
                Some((None, Some(const_name)))
            } else {
                None
            }
        }
        _ => None,
    }
}

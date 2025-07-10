use syn::{Ident, Path, ReturnType, Type, TypePath};

pub trait PathHelper {
    fn get_element_with_inner(&self) -> Option<(Ident, Option<Type>)>;
}

impl PathHelper for ReturnType {
    fn get_element_with_inner(&self) -> Option<(Ident, Option<Type>)> {
        match self {
            ReturnType::Default => None,
            ReturnType::Type(_, boxed_type) => match boxed_type.as_ref() {
                Type::Path(ty_path) => ty_path.get_element_with_inner(),
                _ => None,
            },
        }
    }
}

impl PathHelper for TypePath {
    fn get_element_with_inner(&self) -> Option<(Ident, Option<Type>)> {
        self.path.get_element_with_inner()
    }
}

impl PathHelper for Path {
    fn get_element_with_inner(&self) -> Option<(Ident, Option<Type>)> {
        let seg = self.segments.last()?;

        let inner_type = match &seg.arguments {
            syn::PathArguments::AngleBracketed(args) => {
                args.args.iter().find_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => Some(ty.clone()),
                    _ => None,
                })
            }
            _ => None,
        };

        Some((seg.ident.clone(), inner_type))
    }
}

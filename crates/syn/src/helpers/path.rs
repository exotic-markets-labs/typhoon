use syn::{Expr, ExprLit, Ident, Lit, Path, ReturnType, Type, TypePath};

pub trait PathHelper {
    fn get_element_with_inner(&self) -> Option<(Ident, Option<Type>, Option<usize>)>;
}

impl PathHelper for ReturnType {
    fn get_element_with_inner(&self) -> Option<(Ident, Option<Type>, Option<usize>)> {
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
    fn get_element_with_inner(&self) -> Option<(Ident, Option<Type>, Option<usize>)> {
        self.path.get_element_with_inner()
    }
}

impl PathHelper for Path {
    fn get_element_with_inner(&self) -> Option<(Ident, Option<Type>, Option<usize>)> {
        let seg = self.segments.last()?;
        let (inner_type, inner_const_size) = extract_generic_arguments(&seg.arguments);

        Some((seg.ident.clone(), inner_type, inner_const_size))
    }
}

fn extract_generic_arguments(args: &syn::PathArguments) -> (Option<Type>, Option<usize>) {
    let mut inner_type = None;
    let mut inner_const_size = None;

    if let syn::PathArguments::AngleBracketed(angle_args) = args {
        for arg in &angle_args.args {
            match arg {
                syn::GenericArgument::Type(ty) if inner_type.is_none() => {
                    inner_type = Some(ty.clone());
                }
                syn::GenericArgument::Const(Expr::Lit(ExprLit {
                    lit: Lit::Int(lit), ..
                })) if inner_const_size.is_none() => {
                    if let Ok(val) = lit.base10_parse() {
                        inner_const_size = Some(val);
                    }
                }
                _ => continue,
            }
        }
    }

    (inner_type, inner_const_size)
}

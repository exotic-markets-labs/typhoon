use {
    crate::helpers::PathHelper,
    heck::ToSnakeCase,
    quote::format_ident,
    syn::{FnArg, Ident, Pat, Type},
};

pub enum InstructionArg {
    Type(Type),
    Context(Ident),
}

pub struct Instruction {
    pub name: Ident,
    pub args: Vec<(Ident, InstructionArg)>,
    pub return_data: Option<Type>,
}

impl TryFrom<&syn::ItemFn> for Instruction {
    type Error = syn::Error;

    fn try_from(value: &syn::ItemFn) -> Result<Self, Self::Error> {
        let return_data = value
            .sig
            .output
            .get_element_with_inner()
            .and_then(|(_, inner)| inner);

        let args = value
            .sig
            .inputs
            .iter()
            .filter_map(|fn_arg| {
                let FnArg::Typed(pat_ty) = fn_arg else {
                    return None;
                };

                let Type::Path(ref ty_path) = *pat_ty.ty else {
                    return None;
                };

                let (name, ty) = ty_path.get_element_with_inner()?;

                if name == "ProgramIdArg" || name == "Remaining" {
                    return None;
                }

                let arg_name = extract_name(&pat_ty.pat)
                    .unwrap_or(format_ident!("{}", name.to_string().to_snake_case()));

                if name == "Arg" || name == "BorshArg" {
                    Some(Ok((arg_name, InstructionArg::Type(ty?))))
                } else {
                    //TODO when it will be extractor
                    // let Some(Type::Path(path)) = ty else {
                    //     return Some(Err(syn::Error::new_spanned(&arg_name, "Invalid ty_path.")));
                    // };
                    // let (name, _) = path.get_element_with_inner()?;
                    Some(Ok((arg_name, InstructionArg::Context(name.clone()))))
                }
            })
            .collect::<Result<Vec<_>, syn::Error>>()?;

        Ok(Instruction {
            name: value.sig.ident.clone(),
            args,
            return_data,
        })
    }
}

fn extract_name(pat: &Pat) -> Option<Ident> {
    match pat {
        Pat::Ident(ident) => Some(ident.ident.clone()),
        Pat::TupleStruct(tuple_struct) => {
            let pat = tuple_struct.elems.first()?;
            extract_name(pat)
        }
        _ => None,
    }
}

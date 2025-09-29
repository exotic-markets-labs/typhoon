use {
    crate::{helpers::PathHelper, Encoding},
    heck::ToSnakeCase,
    quote::format_ident,
    syn::{parse::Parser, punctuated::Punctuated, FnArg, Ident, Pat, Type},
};

pub struct InstructionReturnData {
    pub ty: Option<Type>,
    pub encoding: Encoding,
}

pub enum InstructionArg {
    Type { ty: Box<Type>, encoding: Encoding },
    Context(Ident),
}

pub struct Instruction {
    pub name: Ident,
    pub args: Vec<(Ident, InstructionArg)>,
    pub return_data: InstructionReturnData,
}

impl TryFrom<&syn::ItemFn> for Instruction {
    type Error = syn::Error;

    fn try_from(value: &syn::ItemFn) -> Result<Self, Self::Error> {
        let return_data = value
            .sig
            .output
            .get_element_with_inner()
            .and_then(|(_, inner, _)| inner);

        let mut args = Vec::with_capacity(value.sig.inputs.len());
        for fn_arg in &value.sig.inputs {
            let FnArg::Typed(pat_ty) = fn_arg else {
                continue;
            };

            let Type::Path(ref ty_path) = *pat_ty.ty else {
                continue;
            };

            let (name, ty, size) = ty_path
                .get_element_with_inner()
                .ok_or(syn::Error::new_spanned(fn_arg, "Invalid FnArg."))?;

            if name == "ProgramIdArg" || name == "Remaining" {
                continue;
            }

            let arg_name = extract_name(&pat_ty.pat)
                .unwrap_or(format_ident!("{}", name.to_string().to_snake_case()));

            if name == "Arg" || name == "BorshArg" {
                args.push((
                    arg_name,
                    InstructionArg::Type {
                        ty: Box::new(
                            ty.ok_or(syn::Error::new_spanned(fn_arg, "Invalid argument type."))?,
                        ),
                        encoding: Encoding::Bytemuck,
                    },
                ));
            } else if name == "BorshArg" {
                args.push((
                    arg_name,
                    InstructionArg::Type {
                        ty: Box::new(
                            ty.ok_or(syn::Error::new_spanned(fn_arg, "Invalid argument type."))?,
                        ),
                        encoding: Encoding::Borsh,
                    },
                ));
            } else if name == "Array" {
                let size = size.ok_or(syn::Error::new_spanned(fn_arg, "Invalid Array type."))?;
                let ty = ty.ok_or(syn::Error::new_spanned(fn_arg, "Invalid argument type."))?;
                let Type::Path(path) = ty else {
                    return Err(syn::Error::new_spanned(&arg_name, "Invalid ty_path."));
                };
                let (name, _, _) = path
                    .get_element_with_inner()
                    .ok_or(syn::Error::new_spanned(&path, "Invalid Array inner type."))?;
                for i in 0..size {
                    let arg_name = format_ident!("{arg_name}_{i}");
                    args.push((arg_name, InstructionArg::Context(name.clone())));
                }
            } else {
                args.push((arg_name, InstructionArg::Context(name.clone())));
            }
        }

        Ok(Instruction {
            name: value.sig.ident.clone(),
            args,
            return_data: InstructionReturnData {
                ty: return_data,
                encoding: Encoding::Bytemuck,
            },
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

#[derive(Default)]
pub struct InstructionsList(pub Vec<(usize, Ident)>);

impl TryFrom<&syn::ItemMacro> for InstructionsList {
    type Error = syn::Error;

    fn try_from(value: &syn::ItemMacro) -> syn::Result<Self> {
        let instructions = Punctuated::<Ident, syn::Token![,]>::parse_terminated
            .parse2(value.mac.tokens.clone())?;
        Ok(Self(
            instructions
                .iter()
                .enumerate()
                .map(|(i, n)| (i, n.clone()))
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        syn::{parse_quote, ItemFn},
    };

    #[test]
    fn test_instruction_construction() {
        let fn_raw: ItemFn = parse_quote! {
            pub fn instruction_1(ctx: Context1, array: Array<Context2, 2>, arg: Arg<u64>) -> ProgramResult {
                Ok(())
            }
        };
        let ix = Instruction::try_from(&fn_raw).unwrap();

        assert_eq!(ix.name, "instruction_1");
        assert_eq!(ix.args.len(), 4);
        assert_eq!(ix.args[0].0, "ctx");
        assert!(matches!(&ix.args[0].1, InstructionArg::Context(x) if x == "Context1"));
        assert_eq!(ix.args[1].0, "array_0");
        assert!(matches!(&ix.args[1].1, InstructionArg::Context(x) if x == "Context2"));
        assert_eq!(ix.args[2].0, "array_1");
        assert!(matches!(&ix.args[2].1, InstructionArg::Context(x) if x == "Context2"));
        assert_eq!(ix.args[3].0, "arg");
        assert!(ix.return_data.ty.is_none());
        assert!(matches!(ix.return_data.encoding, Encoding::Bytemuck));
    }
}

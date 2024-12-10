use {
    five8_const::decode_32_const,
    syn::{Expr, ExprArray, File, Ident, Item, ItemConst, ItemFn, Stmt, Type},
};

// rename indexer
#[derive(Debug)]
pub struct ParsingContext<'a> {
    pub contexts: Vec<&'a Ident>,
    pub program_id: Option<[u8; 32]>,
    pub instructions: Vec<&'a Ident>,
    pub accounts: Vec<&'a Ident>,
    pub file: &'a File,
    // pub manifest: Manifest, HOME or CARGO_HOME
}
impl<'a> From<&'a File> for ParsingContext<'a> {
    fn from(value: &'a File) -> Self {
        let estimated_size = value.items.len() / 2;
        let mut contexts = Vec::with_capacity(estimated_size);
        let mut accounts = Vec::with_capacity(estimated_size);
        let mut instructions = Vec::with_capacity(estimated_size);
        let mut program_id = None;

        for item in value.items.iter() {
            match item {
                Item::Impl(item_impl) => {
                    if let Some(ident) = extract_ident(item_impl, "HandlerContext") {
                        contexts.push(ident);
                    } else if let Some(ident) = extract_ident(item_impl, "Owner") {
                        accounts.push(ident);
                    }
                }
                Item::Fn(item_fn) => {
                    if let Some(mut ins) = extract_instruction_idents(item_fn) {
                        instructions.append(&mut ins);
                    }
                }
                Item::Const(item_const) => {
                    if let Some(parsed_id) = extract_program_id(item_const) {
                        program_id = Some(parsed_id);
                    }
                }
                _ => (),
            }
        }

        ParsingContext {
            program_id,
            accounts,
            contexts,
            instructions,
            file: value,
        }
    }
}

fn extract_ident<'a>(item_impl: &'a syn::ItemImpl, trait_name: &str) -> Option<&'a Ident> {
    let trait_ = item_impl.trait_.as_ref()?;
    let segment = trait_.1.segments.last()?;

    if segment.ident != trait_name {
        return None;
    }

    match *item_impl.self_ty {
        Type::Path(ref type_path) => Some(&type_path.path.segments.last()?.ident),
        _ => None,
    }
}

fn extract_instruction_idents(item_fn: &ItemFn) -> Option<Vec<&Ident>> {
    if item_fn.sig.ident != "process_instruction" {
        return None;
    }

    let match_expr = item_fn.block.stmts.iter().find_map(|stmt| {
        if let Stmt::Expr(Expr::Match(m), ..) = stmt {
            Some(m)
        } else {
            None
        }
    })?;

    let instructions = match_expr
        .arms
        .iter()
        .filter_map(|arm| {
            let Expr::Try(try_expr) = arm.body.as_ref() else {
                return None;
            };

            let Expr::Call(call) = try_expr.expr.as_ref() else {
                return None;
            };

            let Expr::Path(p) = call.func.as_ref() else {
                return None;
            };
            if p.path.segments.last()?.ident != "handle" {
                return None;
            };

            call.args.last().and_then(|arg| {
                if let Expr::Path(p) = arg {
                    p.path.get_ident()
                } else {
                    None
                }
            })
        })
        .collect();

    Some(instructions)
}

fn extract_program_id(item_const: &ItemConst) -> Option<[u8; 32]> {
    if item_const.ident != "ID" {
        return None;
    }

    let expr_call = match item_const.expr.as_ref() {
        Expr::Call(call) => call,
        _ => return None,
    };

    let first_arg = expr_call.args.first()?;

    match first_arg {
        Expr::Array(array) => parse_array_literal(array),
        Expr::Lit(lit) => {
            if let syn::Lit::Str(str_lit) = &lit.lit {
                Some(decode_32_const(&str_lit.value()))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_array_literal(array: &ExprArray) -> Option<[u8; 32]> {
    if array.elems.len() != 32 {
        return None;
    }

    let mut bytes = [0u8; 32];

    for (i, elem) in array.elems.iter().enumerate() {
        let expr_lit = match elem {
            Expr::Lit(lit) => lit,
            _ => return None,
        };

        let lit_int = match &expr_lit.lit {
            syn::Lit::Int(int) => int,
            _ => return None,
        };

        bytes[i] = lit_int.base10_parse().ok()?;
    }

    Some(bytes)
}

#[cfg(test)]
mod tests {
    use {super::*, syn::parse_quote};

    #[test]
    fn program_id() {
        let parsed: ItemConst = parse_quote!(
            /// The const program ID.
            pub const ID: ::solana_program::pubkey::Pubkey =
                ::solana_program::pubkey::Pubkey::new_from_array([
                    218u8, 7u8, 92u8, 178u8, 255u8, 94u8, 198u8, 129u8, 118u8, 19u8, 222u8, 83u8,
                    11u8, 105u8, 42u8, 135u8, 53u8, 71u8, 119u8, 105u8, 218u8, 71u8, 67u8, 12u8,
                    189u8, 129u8, 84u8, 51u8, 92u8, 74u8, 131u8, 39u8,
                ]);
        );

        let program_id = extract_program_id(&parsed);
        let expected_key = [
            218, 7, 92, 178, 255, 94, 198, 129, 118, 19, 222, 83, 11, 105, 42, 135, 53, 71, 119,
            105, 218, 71, 67, 12, 189, 129, 84, 51, 92, 74, 131, 39,
        ];

        assert_eq!(program_id, Some(expected_key));

        let parsed: ItemConst = parse_quote!(
            ///The const program ID.
            pub const ID: ::pinocchio_pubkey::pinocchio::pubkey::Pubkey =
                ::pinocchio_pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
        );
        let program_id = extract_program_id(&parsed);
        let expected_key = [
            218, 7, 92, 178, 255, 94, 198, 129, 118, 19, 222, 83, 11, 105, 42, 135, 53, 71, 119,
            105, 218, 71, 67, 12, 189, 129, 84, 51, 92, 74, 131, 39,
        ];

        assert_eq!(program_id, Some(expected_key));
    }
}

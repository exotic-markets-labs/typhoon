use {
    codama::{CodamaResult, KorokVisitor},
    std::collections::HashMap,
    syn::{
        parse::Parser, punctuated::Punctuated, Arm, Expr, ExprMatch, Ident, Item, ItemFn,
        ItemMacro, Stmt,
    },
};

pub struct CacheInstructionIdents<'a> {
    cache: &'a mut HashMap<String, usize>,
}

impl<'a> CacheInstructionIdents<'a> {
    pub fn new(cache: &'a mut HashMap<String, usize>) -> Self {
        CacheInstructionIdents { cache }
    }
}

impl KorokVisitor for CacheInstructionIdents<'_> {
    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;

        let maybe_ins = match korok.ast {
            Item::Fn(item_fn) => extract_from_fn(item_fn),
            Item::Macro(item_macro) => extract_from_macro(item_macro),
            _ => None,
        };

        if let Some(ins) = maybe_ins {
            for (dis, fn_name) in ins {
                self.cache.insert(fn_name, dis);
            }
        }

        Ok(())
    }
}

fn extract_from_macro(item_macro: &ItemMacro) -> Option<Vec<(usize, String)>> {
    if !item_macro.mac.path.is_ident("handlers") {
        return None;
    }

    let ins = Punctuated::<Ident, syn::Token![,]>::parse_terminated
        .parse2(item_macro.mac.tokens.clone())
        .ok()?;

    Some(
        ins.iter()
            .enumerate()
            .map(|(i, el)| (i, el.to_string()))
            .collect(),
    )
}

fn extract_from_fn(item_fn: &ItemFn) -> Option<Vec<(usize, String)>> {
    // Only process functions named "process_instruction"
    if item_fn.sig.ident != "process_instruction" {
        return None;
    }

    // Find the match expression in the function body
    let match_expr = find_match_expr(item_fn)?;

    // Extract instruction identifiers from match arms
    let instructions = match_expr
        .arms
        .iter()
        .enumerate()
        .filter_map(|(i, item)| extract_instruction_from_arm(item).map(|el| (i, el)))
        .collect();

    Some(instructions)
}

fn find_match_expr(item_fn: &ItemFn) -> Option<&ExprMatch> {
    item_fn.block.stmts.iter().find_map(|stmt| {
        if let Stmt::Expr(Expr::Match(m), ..) = stmt {
            Some(m)
        } else {
            None
        }
    })
}

fn extract_instruction_from_arm(arm: &Arm) -> Option<String> {
    // Extract try expression
    let try_expr = match arm.body.as_ref() {
        Expr::Try(try_expr) => try_expr,
        _ => return None,
    };

    // Extract call expression
    let call = match try_expr.expr.as_ref() {
        Expr::Call(call) => call,
        _ => return None,
    };

    // Verify the call is to a "handle" function
    let path = match call.func.as_ref() {
        Expr::Path(p) => p,
        _ => return None,
    };
    if path.path.segments.last()?.ident != "handle" {
        return None;
    }

    // Extract instruction identifier from last argument
    call.args.last().and_then(|arg| match arg {
        Expr::Path(p) => p.path.get_ident().map(|el| el.to_string()),
        _ => None,
    })
}

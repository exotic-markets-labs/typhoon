use {
    std::{
        path::Path,
        process::{Command, Stdio},
    },
    typhoon_metadata_extractor::parsing::ParsingContext,
};

pub fn main() {
    use cargo_manifest::Manifest;

    // let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_path = Path::new("/home/aursen/typhoon/examples/counter")
        .join("Cargo.toml")
        .canonicalize()
        .unwrap();
    let manifest = Manifest::from_path(manifest_path).unwrap();

    if let Some(workspace) = manifest.workspace {
        println!("{workspace:?}");
        println!("WORKSPACE");
        // Workspace program
        // for member in workspace.members {
        //     // TODO with one and cache what needed.
        // }
    } else {
        // println!("unique one");

        let command = Command::new("cargo") //TODO do it without expand
            .arg("expand")
            .arg("--lib")
            .arg(format!("--package={}", manifest.package.unwrap().name))
            .stderr(Stdio::inherit())
            .output()
            .unwrap()
            .stdout;
        let content = String::from_utf8(command).unwrap();
        let file = syn::parse_file(&content).unwrap();
        let context = ParsingContext::from(&file);

        println!("{context:?}");

        // let context_idents: Vec<&Ident> = file
        //     .items
        //     .iter()
        //     .filter_map(|item| match item {
        //         syn::Item::Impl(item_impl) => extract_context_ident(item_impl),
        //         _ => None,
        //     })
        //     .collect();

        // let instruction_idents = extract_instruction_idents(&file.items);

        // fn extract_instruction_idents<'a>(items: &'a [Item]) -> Vec<&'a Ident> {
        //     let mut instruction_idents = Vec::new();
        //     for item in items {
        //         if let Item::Fn(fn_truc) = item {
        //             if fn_truc.sig.ident == "process_instruction" {
        //                 for stmt in &fn_truc.block.stmts {
        //                     if let syn::Stmt::Expr(Expr::Match(match_handler), ..) = stmt {
        //                         for arm in &match_handler.arms {
        //                             let Expr::Try(try_handle) = arm.body.as_ref() else {
        //                                 continue;
        //                             };

        //                             let Expr::Call(handle) = try_handle.expr.as_ref() else {
        //                                 continue;
        //                             };

        //                             if let Expr::Path(fn_path) = handle.func.as_ref() {
        //                                 if fn_path
        //                                     .path
        //                                     .segments
        //                                     .last()
        //                                     .map_or(false, |seg| seg.ident == "handle")
        //                                 {
        //                                     if let Some(Expr::Path(instruction_name)) =
        //                                         handle.args.last()
        //                                     {
        //                                         if let Some(ident) =
        //                                             instruction_name.path.get_ident()
        //                                         {
        //                                             instruction_idents.push(ident);
        //                                         }
        //                                     }
        //                                 }
        //                             }
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     instruction_idents
        // }

        // let accounts: Vec<&Ident> = file
        //     .items
        //     .iter()
        //     .filter_map(|item| match item {
        //         Item::Impl(item_impl) => extract_context_ident(item_impl),
        //         _ => None,
        //     })
        //     .collect();

        // println!("{accounts:?}");
        // fn extract_context_ident(item_impl: &syn::ItemImpl) -> Option<&Ident> {
        //     let trait_: &(Option<syn::token::Not>, syn::Path, syn::token::For) =
        //         item_impl.trait_.as_ref()?;
        //     let segment = trait_.1.segments.last()?;

        //     if segment.ident != "Owner" {
        //         return None;
        //     }

        //     match *item_impl.self_ty {
        //         syn::Type::Path(ref type_path) => Some(&type_path.path.segments.last()?.ident),
        //         _ => None,
        //     }
        // }

        // println!("{instruction_idents:?}");
        // let instruction_idents: Vec<&Ident> = file
        //     .items
        //     .iter()
        //     .map(|item| {
        //         // panic!("{:?}", );
        //         // Ident::new("random", Span::call_site())
        //         // if let Item::Fn(item_fn) = item {
        //         //     item_fn.sig.ident == ""

        //         //     // item_fn.block.brace_token
        //         //     // if item_fn.sig.ident == "processor" {
        //         //     // }
        //         // }
        //     })
        //     .collect();

        // let context: Vec<Context> = file
        //     .items
        //     .iter()
        //     .filter_map(|item| {
        //         if let syn::Item::Struct(item_struct) = item {
        //             if context_idents.contains(&&item_struct.ident) {
        //                 let fields = item_struct
        //                     .fields
        //                     .iter()
        //                     .map(|f| {
        //                         let mut docs = Docs::default();
        //                         f.attrs.iter().for_each(|attr| docs.visit_attribute(attr));

        //                         let account = Account {
        //                             name: f.ident.as_ref().unwrap(),
        //                             docs,
        //                         };

        //                         ContextField::Account(account)
        //                     })
        //                     .collect();

        //                 return Some(Context {
        //                     name: &item_struct.ident,
        //                     args: None,
        //                     fields,
        //                 });
        //             }
        //         }
        //         None
        //     })
        //     .collect();

        // println!("{context:?}")
    }
}
// fn extract_instruction_idents()

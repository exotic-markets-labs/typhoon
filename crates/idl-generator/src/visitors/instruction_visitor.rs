use {
    codama::{
        CamelCaseString, CodamaError, ConstantDiscriminatorNode, ConstantValueNode,
        DefinedTypeLinkNode, DiscriminatorNode, Docs, InstructionAccountNode,
        InstructionArgumentNode, InstructionNode, KorokVisitor, Node, NumberFormat::U8,
        NumberTypeNode, NumberValueNode, TypeNode,
    },
    codama_koroks::{StructKorok, UnsupportedItemKorok},
    codama_syn_helpers::extensions::PathExtension,
    std::collections::HashMap,
    syn::{ExprLit, FnArg, GenericArgument, Item, Lit, Pat, Type},
};

pub struct InstructionVisitor<'a> {
    ixs: &'a HashMap<String, usize>,
    contexts: HashMap<String, Vec<InstructionAccountNode>>,
}

impl<'a> InstructionVisitor<'a> {
    pub fn new(ixs: &'a HashMap<String, usize>) -> Self {
        Self {
            ixs,
            contexts: HashMap::new(),
        }
    }
}

impl KorokVisitor for InstructionVisitor<'_> {
    fn visit_struct(&mut self, korok: &mut StructKorok) -> codama::CodamaResult<()> {
        let name = korok.ast.ident.to_string();
        let accounts = korok.fields.all.iter().filter_map(|f| {
            if let Some(Node::InstructionAccount(account)) = &f.node {
                Some(account)
            } else {
                None
            }
        });
        self.contexts.insert(name, accounts.cloned().collect());

        Ok(())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut UnsupportedItemKorok,
    ) -> codama::CodamaResult<()> {
        let UnsupportedItemKorok {
            ast: Item::Fn(item_fn),
            node: None,
            ..
        } = korok
        else {
            return Ok(());
        };

        let mut accounts: Vec<InstructionAccountNode> = Vec::new();
        let mut arguments: Vec<InstructionArgumentNode> = Vec::new();

        for arg in &item_fn.sig.inputs {
            let FnArg::Typed(pat_ty) = arg else { continue };
            let Type::Path(ref ty_path) = *pat_ty.ty else {
                continue;
            };

            match &ty_path.path.segments.last().unwrap().arguments {
                syn::PathArguments::None => (),
                syn::PathArguments::AngleBracketed(bracket_args) => {
                    let Some(GenericArgument::Type(arg_ty)) = bracket_args.args.first() else {
                        return Err(CodamaError::Compilation(syn::Error::new_spanned(
                            ty_path,
                            "Invalid argument type",
                        )));
                    };

                    let Type::Path(arg_path) = arg_ty else {
                        return Err(CodamaError::Compilation(syn::Error::new_spanned(
                            ty_path,
                            "The argument is not a path type",
                        )));
                    };

                    let struct_name = arg_path.path.last_str();
                    let name = if let Pat::Lit(ExprLit {
                        lit: Lit::Str(ref name),
                        ..
                    }) = *pat_ty.pat
                    {
                        CamelCaseString::new(name.value())
                    } else {
                        CamelCaseString::new(&struct_name)
                    };

                    arguments.push(InstructionArgumentNode {
                        name,
                        docs: Docs::new(),
                        r#type: TypeNode::Link(DefinedTypeLinkNode::new(struct_name)),
                        default_value: None,
                        default_value_strategy: None,
                    });
                }
                syn::PathArguments::Parenthesized(_) => (),
            }

            let Some(name) = ty_path.path.get_ident() else {
                continue;
            };

            if let Some(context_accounts) = self.contexts.get(&name.to_string()) {
                accounts.append(&mut context_accounts.clone());
            }
        }

        let name = item_fn.sig.ident.to_string();
        let discriminator_val = self.ixs.get(&name).cloned().unwrap_or_default();
        let node = InstructionNode {
            name: CamelCaseString::new(name),
            discriminators: vec![DiscriminatorNode::Constant(ConstantDiscriminatorNode::new(
                ConstantValueNode::new(
                    NumberTypeNode::le(U8),
                    NumberValueNode::new(discriminator_val as u32),
                ),
                0,
            ))],
            accounts,
            arguments,
            ..Default::default()
        };
        //TODO remaining accounts
        korok.node = Some(Node::Instruction(node));

        Ok(())
    }
}

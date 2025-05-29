use {
    codama::{
        CamelCaseString, ConstantDiscriminatorNode, ConstantValueNode, DiscriminatorNode,
        InstructionAccountNode, InstructionNode, KorokVisitor, Node, NumberFormat::U8,
        NumberTypeNode, NumberValueNode,
    },
    codama_koroks::{StructKorok, UnsupportedItemKorok},
    std::collections::HashMap,
    syn::{FnArg, Item, Type},
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

        for arg in &item_fn.sig.inputs {
            let FnArg::Typed(pat_ty) = arg else { continue };
            let Type::Path(ref ty_path) = *pat_ty.ty else {
                continue;
            };

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
            arguments: vec![],
            ..Default::default()
        };
        korok.node = Some(Node::Instruction(node));

        Ok(())
    }
}

use {
    codama::{
        CamelCaseString, CombineTypesVisitor, ComposeVisitor, DefinedTypeLinkNode, KorokVisitor,
        LinkNode, Node, SetBorshTypesVisitor, TypeNode,
    },
    codama_koroks::{FieldKorok, StructKorok, UnsupportedItemKorok},
    std::collections::HashSet,
};

pub struct SetDefinedTypesVisitor {
    names: HashSet<String>,
    visitor: Box<dyn KorokVisitor>,
}

impl Default for SetDefinedTypesVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl SetDefinedTypesVisitor {
    pub fn new() -> Self {
        Self {
            names: HashSet::new(),
            visitor: Box::new(
                ComposeVisitor::new()
                    .with(SetBorshTypesVisitor::new())
                    .with(CombineTypesVisitor::new()),
            ),
        }
    }
}

impl KorokVisitor for SetDefinedTypesVisitor {
    fn visit_struct(&mut self, korok: &mut StructKorok) -> codama::CodamaResult<()> {
        if korok.node.is_some() {
            return Ok(());
        }

        let name = CamelCaseString::new(korok.ast.ident.to_string());
        if self.names.contains(name.as_str()) {
            self.visitor.visit_struct(korok)?;
        }

        Ok(())
    }

    fn visit_field(&mut self, korok: &mut FieldKorok) -> codama::CodamaResult<()> {
        let Some(Node::Link(LinkNode::DefinedType(DefinedTypeLinkNode { ref name, .. }))) =
            korok.node
        else {
            return Ok(());
        };

        self.names.insert(name.to_string());

        Ok(())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut UnsupportedItemKorok,
    ) -> codama::CodamaResult<()> {
        let Some(Node::Instruction(ref ix)) = korok.node else {
            return Ok(());
        };

        for arg in &ix.arguments {
            if let TypeNode::Link(DefinedTypeLinkNode { ref name, .. }) = arg.r#type {
                self.names.insert(name.to_string());
            }
        }

        Ok(())
    }
}

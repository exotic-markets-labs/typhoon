use {
    codama::{InstructionNode, KorokVisitor, Node},
    codama_koroks::UnsupportedItemKorok,
    hashbrown::HashMap,
    syn::Item,
};

pub struct ApplyInstructionVisitor {
    pub instructions: HashMap<String, InstructionNode>,
}

impl ApplyInstructionVisitor {
    pub fn new(instructions: HashMap<String, InstructionNode>) -> Self {
        ApplyInstructionVisitor { instructions }
    }
}

impl KorokVisitor for ApplyInstructionVisitor {
    fn visit_unsupported_item(
        &mut self,
        korok: &mut UnsupportedItemKorok,
    ) -> codama::CodamaResult<()> {
        let UnsupportedItemKorok {
            ast: Item::Fn(item_fn),
            ..
        } = korok
        else {
            return Ok(());
        };

        if let Some(ix) = self.instructions.remove(&item_fn.sig.ident.to_string()) {
            korok.node = Some(Node::Instruction(ix));
        }

        Ok(())
    }
}

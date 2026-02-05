use {
    codama::{InstructionNode, KorokVisitor, Node},
    codama_koroks::UnsupportedItemKorok,
    hashbrown::HashMap,
    syn::Item,
    typhoon_syn::Instruction,
};

#[derive(Default)]
pub struct InstructionVisitor {
    ix_cache: Option<String>,
}

impl InstructionVisitor {
    pub fn new() -> Self {
        InstructionVisitor::default()
    }
}

impl KorokVisitor for InstructionVisitor {
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

        if let Ok(ix) = Instruction::try_from(item_fn) {
            // korok.node = Some(Instruction)
        }

        Ok(())
    }
}

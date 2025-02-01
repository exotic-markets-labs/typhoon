use {
    codama::{CodamaResult, CrateKorok, KorokVisitor, Node, ProgramNode, UnsupportedItemKorok},
    codama_syn_helpers::extensions::PathExtension,
};

#[derive(Default)]
pub struct SetProgramIdVisitor {
    identified_public_key: Option<String>,
}

impl SetProgramIdVisitor {
    pub fn new() -> Self {
        SetProgramIdVisitor::default()
    }
}

impl KorokVisitor for SetProgramIdVisitor {
    fn visit_crate(&mut self, korok: &mut CrateKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;

        // Get a mutable reference to the program to update its metadata.
        let program = match &mut korok.node {
            // Use the primary program of the root node if set.
            Some(Node::Root(root)) => &mut root.program,
            // Use the existing program node if set.
            Some(Node::Program(program)) => program,
            // If no node is set, create a new default program node.
            None => {
                korok.node = Some(ProgramNode::default().into());
                if let Some(Node::Program(program)) = &mut korok.node {
                    program
                } else {
                    unreachable!()
                }
            }
            // Don't update the node if it is set to anything else.
            _ => return Ok(()),
        };

        if program.public_key.is_empty() {
            if let Some(public_key) = &self.identified_public_key {
                program.public_key = public_key.into()
            }
        }

        Ok(())
    }

    fn visit_unsupported_item(&mut self, korok: &mut UnsupportedItemKorok) -> CodamaResult<()> {
        let syn::Item::Macro(syn::ItemMacro { mac, .. }) = korok.ast else {
            return Ok(());
        };

        if let ("" | "typhoon_program_id_macro", "program_id") =
            (mac.path.prefix().as_str(), mac.path.last_str().as_str())
        {
            self.identified_public_key = Some(mac.tokens.to_string().replace("\"", ""));
        };

        Ok(())
    }
}

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
        Self::default()
    }

    fn update_program_node(&mut self, program: &mut ProgramNode) {
        if program.public_key.is_empty() {
            if let Some(public_key) = &self.identified_public_key {
                program.public_key = public_key.into();
            }
        }
    }

    fn handle_macro(&mut self, mac: &syn::Macro) {
        if let ("" | "typhoon_program_id_macro", "program_id") =
            (mac.path.prefix().as_str(), mac.path.last_str().as_str())
        {
            self.identified_public_key = Some(mac.tokens.to_string().replace("\"", ""));
        }
    }
}

impl KorokVisitor for SetProgramIdVisitor {
    fn visit_crate(&mut self, korok: &mut CrateKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;

        let program = match &mut korok.node {
            Some(Node::Root(root)) => &mut root.program,
            Some(Node::Program(program)) => program,
            None => {
                korok.node = Some(ProgramNode::default().into());
                match &mut korok.node {
                    Some(Node::Program(program)) => program,
                    _ => unreachable!(),
                }
            }
            _ => return Ok(()),
        };

        self.update_program_node(program);
        Ok(())
    }

    fn visit_unsupported_item(&mut self, korok: &mut UnsupportedItemKorok) -> CodamaResult<()> {
        if let syn::Item::Macro(syn::ItemMacro { mac, .. }) = korok.ast {
            self.handle_macro(mac);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        codama::{CrateStore, KorokVisitable},
        quote::quote,
    };

    #[test]
    fn it_gets_program_ids_from_the_declare_id_macro() {
        let store = CrateStore::hydrate(quote! {
            program_id!("MyProgramAddress1111111111111111111111111");
        })
        .unwrap();
        let mut korok = CrateKorok::parse(&store).unwrap();
        korok.accept(&mut SetProgramIdVisitor::new()).unwrap();

        let Some(Node::Program(program)) = korok.node else {
            panic!("Expected program node");
        };
        assert_eq!(
            program.public_key,
            "MyProgramAddress1111111111111111111111111"
        );
    }
}

use {
    codama::{CodamaResult, CrateKorok, ErrorNode, KorokVisitor, Node, ProgramNode},
    typhoon_syn::Errors,
};

pub struct SetErrorsVisitor {
    name: String,
    errors: Option<Vec<ErrorNode>>,
}

impl SetErrorsVisitor {
    pub fn new(name: impl ToString) -> Self {
        SetErrorsVisitor {
            name: name.to_string(),
            errors: None,
        }
    }

    fn update_program_node(&mut self, program: &mut ProgramNode) {
        if program.errors.is_empty() {
            if let Some(errors) = self.errors.take() {
                program.errors = errors;
            }
        }
    }
}

impl KorokVisitor for SetErrorsVisitor {
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

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> codama::CodamaResult<()> {
        if korok.ast.ident != self.name {
            return Ok(());
        }

        if !korok.attributes.has_derive(&[""], "TyphoonError") {
            return Ok(());
        }

        let errors = Errors::try_from(korok.ast)?;
        //TODO inject Typhoon errors here
        self.errors = Some(
            errors
                .variants
                .into_iter()
                .map(|v| ErrorNode::new(v.name.to_string(), v.discriminant as usize, v.msg))
                .collect(),
        );

        Ok(())
    }
}

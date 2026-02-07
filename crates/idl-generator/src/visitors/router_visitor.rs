use {
    codama::{
        CamelCaseString, ConstAst, ConstantDiscriminatorNode, ConstantValueNode, DiscriminatorNode,
        InstructionNode, KorokVisitor, Node, NumberFormat::U8, NumberTypeNode, NumberValueNode,
        ProgramNode,
    },
    hashbrown::HashMap,
    typhoon_syn::{Context, Instruction, InstructionsList},
};

#[derive(Default)]
pub struct RouterVisitor {
    pub instruction_list: InstructionsList,
    pub errors_name: String,
    pub instructions: HashMap<String, Instruction>,
    pub contexts: HashMap<String, Context>,
}

impl RouterVisitor {
    pub fn new() -> Self {
        RouterVisitor::default()
    }
}

impl KorokVisitor for RouterVisitor {
    fn visit_const(&mut self, korok: &mut codama_koroks::ConstKorok) -> codama::CodamaResult<()> {
        let ConstAst::Item(item_const) = korok.ast else {
            return Ok(());
        };

        if item_const.ident == "ROUTER" {
            let program = match korok
                .node
                .get_or_insert_with(|| ProgramNode::default().into())
            {
                Node::Root(root) => &mut root.program,
                Node::Program(program) => program,
                _ => return Ok(()),
            };

            let ix_list = InstructionsList::try_from(item_const)?;
            program.instructions = ix_list
                .0
                .iter()
                .map(|(dis, name)| InstructionNode {
                    discriminators: vec![DiscriminatorNode::Constant(
                        ConstantDiscriminatorNode::new(
                            ConstantValueNode::new(
                                NumberTypeNode::le(U8),
                                NumberValueNode::new(*dis as u8),
                            ),
                            0,
                        ),
                    )],
                    name: CamelCaseString::new(name.to_string()),
                    ..Default::default()
                })
                .collect();
        }

        Ok(())
    }
}

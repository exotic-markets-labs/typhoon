use codama::{
    CamelCaseString, InstructionNode, KorokVisitor, Node, ProgramNode, UnsupportedItemKorok,
};
use syn::Item;
use typhoon_syn::Instruction;

#[derive(Default)]
pub struct InstructionResolver {
    router_cache: Option<ProgramNode>,
    context_cache: Option<Vec<InstructionNode>>,
}

impl InstructionResolver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for InstructionResolver {
    fn visit_const(&mut self, korok: &mut codama_koroks::ConstKorok) -> codama::CodamaResult<()> {
        let Some(Node::Program(program)) = korok.node.take() else {
            return Ok(());
        };

        self.router_cache = Some(program);

        Ok(())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> codama::CodamaResult<()> {
        let Some(Node::Instruction(instruction)) = &korok.node else {
            return Ok(());
        };

        let cache = self.context_cache.get_or_insert_with(Vec::new);
        cache.push(instruction.clone());
        Ok(())
    }

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

        let Some(router_cache) = self.router_cache.as_mut() else {
            return Ok(());
        };

        // if let
        let ix = Instruction::try_from(item_fn)?;

        // item_fn.sig.inputs.

        // let Some(ix_cache) = self.ix_cache.as_ref() else {
        //     return Ok(());
        // };

        // let fn_name = CamelCaseString::new(item_fn.sig.ident.to_string());
        // let Some(ix) = ix_cache.iter().find(|ix| ix.name == fn_name) else {
        //     return Ok(());
        // };

        // korok.node = Some(Node::Instruction(ix.clone()));

        Ok(())
    }
}

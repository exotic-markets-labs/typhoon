use {
    crate::visitors::SetBorshTypesVisitor,
    base64::{prelude::BASE64_STANDARD, Engine},
    codama::{
        AccountNode, CamelCaseString, CombineTypesVisitor, ComposeVisitor,
        ConstantDiscriminatorNode, ConstantValueNode, DefinedTypeNode, DiscriminatorNode, Docs,
        KorokVisitor, Node, SetLinkTypesVisitor, TypeNode,
    },
    typhoon_discriminator::DiscriminatorBuilder,
    typhoon_syn::Docs as TyphoonDocs,
};

pub struct SetAccountVisitor {
    visitor: Box<dyn KorokVisitor>,
}

impl Default for SetAccountVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl SetAccountVisitor {
    pub fn new() -> Self {
        Self {
            visitor: Box::new(
                ComposeVisitor::new()
                    .with(SetBorshTypesVisitor::new())
                    .with(SetLinkTypesVisitor::new())
                    .with(CombineTypesVisitor::new()),
            ),
        }
    }
}

impl KorokVisitor for SetAccountVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> codama::CodamaResult<()> {
        if !korok.attributes.has_derive(&[""], "AccountState") {
            return Ok(());
        };

        self.visitor.visit_struct(korok)?;

        let Some(Node::DefinedType(DefinedTypeNode {
            r#type: TypeNode::Struct(ty),
            ..
        })) = korok.node.take()
        else {
            return Ok(());
        };

        let dis = DiscriminatorBuilder::new(&korok.ast.ident.to_string()).build();

        korok.node = Some(Node::Account(AccountNode {
            name: CamelCaseString::new(korok.ast.ident.to_string()),
            size: None,
            docs: Docs::from(TyphoonDocs::from(korok.ast.attrs.as_slice()).into_vec()),
            data: codama::NestedTypeNode::Value(ty),
            pda: None,
            discriminators: vec![DiscriminatorNode::Constant(ConstantDiscriminatorNode::new(
                ConstantValueNode::bytes(
                    codama::BytesEncoding::Base64,
                    BASE64_STANDARD.encode(dis),
                ),
                0,
            ))],
        }));

        Ok(())
    }
}

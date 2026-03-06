use {
    codama::{
        CamelCaseString, CombineTypesVisitor, DefinedTypeLinkNode, DefinedTypeNode, KorokVisitor,
        Node, ProgramNode, RegisteredTypeNode, RootNode, TypeNode,
    },
    hashbrown::HashSet,
};

#[derive(Default)]
pub struct DefineTypeResolver {
    combine_types: CombineTypesVisitor,
    referenced_defined_types: HashSet<String>,
    resolved_defined_types: Vec<DefinedTypeNode>,
    resolved_defined_type_names: HashSet<String>,
    phase: VisitPhase,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum VisitPhase {
    #[default]
    CollectLinks,
    ApplyTypes,
}

impl DefineTypeResolver {
    pub fn new() -> Self {
        Self::default()
    }

    fn collect_from_type_node(&mut self, type_node: &TypeNode) {
        let TypeNode::Link(DefinedTypeLinkNode { name, .. }) = type_node else {
            return;
        };

        self.referenced_defined_types.insert(name.to_string());
    }

    fn is_referenced_defined_type(&self, name: &str) -> bool {
        let type_name = CamelCaseString::new(name);
        self.referenced_defined_types.contains(type_name.as_str())
    }

    fn maybe_collect_resolved_defined_type(&mut self, node: Option<&Node>) {
        let Some(Node::DefinedType(defined_type)) = node else {
            return;
        };

        let name = defined_type.name.to_string();
        if self.resolved_defined_type_names.insert(name) {
            self.resolved_defined_types.push(defined_type.clone());
        }
    }

    fn append_resolved_defined_types_to_root(&mut self, korok: &mut codama_koroks::RootKorok) {
        let Some(Node::Root(root)) = korok.node.as_mut() else {
            return;
        };

        let mut existing_names: HashSet<String> = root
            .program
            .defined_types
            .iter()
            .map(|defined_type| defined_type.name.to_string())
            .collect();

        for defined_type in self.resolved_defined_types.drain(..) {
            if existing_names.insert(defined_type.name.to_string()) {
                root.program.defined_types.push(defined_type);
            }
        }
    }
}

impl KorokVisitor for DefineTypeResolver {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> codama::CodamaResult<()> {
        self.referenced_defined_types.clear();
        self.resolved_defined_types.clear();
        self.resolved_defined_type_names.clear();

        if let Some(Node::Root(RootNode {
            program: ProgramNode {
                ref instructions, ..
            },
            ..
        })) = korok.node
        {
            for ix in instructions {
                for arg in &ix.arguments {
                    self.collect_from_type_node(&arg.r#type);
                }
            }
        }

        for phase in [VisitPhase::CollectLinks, VisitPhase::ApplyTypes] {
            self.phase = phase;
            for crate_korok in &mut korok.crates {
                self.visit_crate(crate_korok)?;
            }
        }

        self.append_resolved_defined_types_to_root(korok);

        Ok(())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> codama::CodamaResult<()> {
        match self.phase {
            VisitPhase::CollectLinks => self.visit_children(korok),
            VisitPhase::ApplyTypes => {
                if self.is_referenced_defined_type(&korok.ast.ident.to_string()) {
                    self.combine_types.visit_struct(korok)?;
                    self.maybe_collect_resolved_defined_type(korok.node.as_ref());
                }
                Ok(())
            }
        }
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> codama::CodamaResult<()> {
        match self.phase {
            VisitPhase::CollectLinks => self.visit_children(korok),
            VisitPhase::ApplyTypes => {
                if self.is_referenced_defined_type(&korok.ast.ident.to_string()) {
                    self.combine_types.visit_enum(korok)?;
                    self.maybe_collect_resolved_defined_type(korok.node.as_ref());
                }
                Ok(())
            }
        }
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> codama::CodamaResult<()> {
        if self.phase != VisitPhase::CollectLinks {
            return Ok(());
        }

        let Some(Node::Type(RegisteredTypeNode::StructField(ref field))) = korok.node else {
            return Ok(());
        };
        self.collect_from_type_node(&field.r#type);

        Ok(())
    }
}

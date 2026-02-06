use {
    crate::{helpers::AttributesHelper, utils::extract_type},
    codama::{
        CamelCaseString, DefinedTypeLinkNode, Docs, InstructionAccountNode,
        InstructionArgumentNode, InstructionNode, IsAccountSigner, KorokVisitor, Node,
        StructFieldTypeNode, StructTypeNode, TypeNode,
    },
    typhoon_syn::{Arguments, InstructionAccount},
};

#[derive(Default)]
pub struct ContextVisitor {
    context_name: Option<String>,
}

impl ContextVisitor {
    pub fn new() -> Self {
        ContextVisitor::default()
    }
}

impl KorokVisitor for ContextVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> codama::CodamaResult<()> {
        let previous_context = self.context_name.clone();
        let is_context = korok.attributes.has_attribute("context");
        if is_context {
            self.context_name = Some(korok.ast.ident.to_string());
        } else {
            self.context_name = None;
        }
        self.visit_children(korok)?;
        if is_context {
            let mut accounts = Vec::with_capacity(korok.ast.fields.len());
            let mut arguments = Vec::with_capacity(1);
            for field in &korok.fields {
                match &field.node {
                    Some(Node::InstructionAccount(account)) => accounts.push(account.clone()),
                    Some(Node::InstructionArgument(argument)) => arguments.push(argument.clone()),
                    _ => {}
                }
            }

            if !accounts.is_empty() || !arguments.is_empty() {
                korok.node = Some(Node::Instruction(InstructionNode {
                    name: CamelCaseString::new(korok.ast.ident.to_string()),
                    accounts,
                    arguments,
                    ..Default::default()
                }));
            }
        }
        self.context_name = previous_context;
        Ok(())
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> codama::CodamaResult<()> {
        let Some(ref context_name) = self.context_name else {
            return Ok(());
        };

        if let Some(attrs) = korok.attributes.0.iter().find(|el| el.name() == "args") {
            let args = Arguments::try_from(attrs.ast())?;
            korok.node = Some(Node::InstructionArgument(InstructionArgumentNode {
                name: CamelCaseString::new(format!("{}_args", context_name)),
                r#type: match args {
                    Arguments::Values(arguments) => TypeNode::Struct(StructTypeNode {
                        fields: arguments
                            .iter()
                            .map(|el| {
                                Ok(StructFieldTypeNode {
                                    name: CamelCaseString::new(el.name.to_string()),
                                    default_value_strategy: None,
                                    docs: Docs::new(),
                                    r#type: extract_type(&el.ty)?,
                                    default_value: None,
                                })
                            })
                            .collect::<Result<_, syn::Error>>()?,
                    }),
                    Arguments::Struct(ident) => {
                        TypeNode::Link(DefinedTypeLinkNode::new(ident.to_string()))
                    }
                },
                default_value_strategy: None,
                docs: Docs::new(),
                default_value: None,
            }));
        } else {
            let account = InstructionAccount::try_from(korok.ast)?;
            korok.node = Some(Node::InstructionAccount(InstructionAccountNode {
                default_value: None,
                docs: Docs::from(account.docs.clone()),
                is_optional: account.meta.is_optional,
                is_signer: if account.meta.is_optional && account.meta.is_signer {
                    IsAccountSigner::Either
                } else {
                    account.meta.is_signer.into()
                },
                is_writable: account.meta.is_mutable,
                name: CamelCaseString::new(account.name.to_string()),
            }));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        codama::{CodamaResult, IsAccountSigner, KorokVisitable, Node, StructKorok},
        syn::{parse_quote, Item},
    };

    #[test]
    fn test_visit_field_in_context() -> CodamaResult<()> {
        let item: Item = parse_quote! {
            #[context]
            pub struct Initialize {
                /// Pays for the account creation.
                pub payer: Mut<Signer>,
                /// Optional authority account.
                pub optional_authority: Option<Signer>,
            }
        };

        let mut korok = StructKorok::parse(&item)?;
        let mut visitor = ContextVisitor::new();
        korok.accept(&mut visitor)?;

        assert!(matches!(korok.node, Some(Node::Instruction(_))));

        let Some(Node::InstructionAccount(payer)) = &korok.fields[0].node else {
            panic!("Expected InstructionAccount node for payer");
        };
        assert_eq!(payer.name.as_str(), "payer");
        assert!(payer.is_writable);
        assert_eq!(payer.is_signer, IsAccountSigner::True);
        assert!(!payer.is_optional);
        assert_eq!(payer.docs[0], "Pays for the account creation.");

        let Some(Node::InstructionAccount(optional)) = &korok.fields[1].node else {
            panic!("Expected InstructionAccount node for optional_authority");
        };
        assert_eq!(optional.name.as_str(), "optionalAuthority");
        assert!(!optional.is_writable);
        assert_eq!(optional.is_signer, IsAccountSigner::Either);
        assert!(optional.is_optional);
        assert_eq!(optional.docs[0], "Optional authority account.");

        Ok(())
    }

    #[test]
    fn test_visit_field_outside_context() -> CodamaResult<()> {
        let item: Item = parse_quote! {
            pub struct NotAContext {
                pub payer: Mut<Signer>,
            }
        };

        let mut korok = StructKorok::parse(&item)?;
        let mut visitor = ContextVisitor::new();
        korok.accept(&mut visitor)?;

        assert!(korok.fields[0].node.is_none());
        Ok(())
    }
}

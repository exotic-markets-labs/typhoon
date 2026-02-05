use {
    crate::{
        utils::extract_type,
        visitors::{
            ContextVisitor, InstructionVisitor, RouterVisitor, SetAccountVisitor, SetErrorsVisitor,
            SetProgramIdVisitor,
        },
    },
    codama::{
        get_type_node, ApplyTypeModifiersVisitor, ApplyTypeOverridesVisitor, CamelCaseString,
        CodamaResult, CombineModulesVisitor, ComposeVisitor, ConstantDiscriminatorNode,
        ConstantValueNode, DefinedTypeLinkNode, DiscriminatorNode, Docs, IdentifyFieldTypesVisitor,
        InstructionAccountNode, InstructionArgumentNode, InstructionNode,
        InstructionOptionalAccountStrategy, IsAccountSigner, NumberFormat::U8, NumberTypeNode,
        NumberValueNode, SetDefaultValuesVisitor, SetDefinedTypesVisitor, SetPdasVisitor,
        SetProgramMetadataVisitor, StructFieldTypeNode, StructTypeNode, TypeNode,
    },
    codama_korok_plugins::KorokPlugin,
    codama_korok_visitors::KorokVisitable,
    hashbrown::HashMap,
    syn::{Error, Type},
    typhoon_syn::{Arguments, InstructionArg},
};

pub struct TyphoonPlugin;

impl KorokPlugin for TyphoonPlugin {
    fn on_initialized(&self, visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        visitable.accept(&mut RouterVisitor::new())?;
        Ok(())
    }

    fn on_fields_set(&self, visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        visitable.accept(&mut IdentifyFieldTypesVisitor::new())?;
        visitable.accept(&mut ApplyTypeOverridesVisitor::new())?;
        visitable.accept(&mut ApplyTypeModifiersVisitor::new())?;
        visitable.accept(&mut SetDefaultValuesVisitor::new())?;
        Ok(())
    }

    fn on_program_items_set(&self, visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        visitable.accept(&mut SetDefinedTypesVisitor::new())?;
        visitable.accept(&mut ContextVisitor::new())?;
        // visitable.accept(&mut SetPdasVisitor::new())?; //TODO
        visitable.accept(&mut SetAccountVisitor::new())?;
        // visitable.accept(&mut SetInstructionsVisitor::new())?; //TODO
        visitable.accept(&mut SetErrorsVisitor::new())?;
        Ok(())
    }

    fn on_root_node_set(&self, visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        visitable.accept(&mut SetProgramIdVisitor::new())?;
        visitable.accept(&mut SetProgramMetadataVisitor::new())?;
        visitable.accept(&mut CombineModulesVisitor::new())?;
        Ok(())
    }
}

fn resolve_instructions(program: &RouterVisitor) -> CodamaResult<HashMap<String, InstructionNode>> {
    let mut result = HashMap::new();
    for (dis, ix) in &program.instruction_list.0 {
        let name = ix.to_string();
        let ix = program
            .instructions
            .get(&name)
            .ok_or(syn::Error::new_spanned(
                ix,
                "Cannot find the correct Instruction.",
            ))?;
        let mut accounts = Vec::new();
        let mut arguments = Vec::new();

        for (arg_name, arg_value) in &ix.args {
            match arg_value {
                InstructionArg::Context(ctx) => {
                    let context = program
                        .contexts
                        .get(&ctx.to_string())
                        .ok_or(syn::Error::new_spanned(ctx, ""))?;

                    for account in &context.accounts {
                        accounts.push(InstructionAccountNode {
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
                        });
                    }

                    if let Some(args) = &context.arguments {
                        arguments.push(InstructionArgumentNode {
                            name: CamelCaseString::new(format!("{}_args", context.name)),
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
                        });
                    }
                }
                InstructionArg::Type { ty, .. } => {
                    arguments.push(InstructionArgumentNode {
                        name: CamelCaseString::new(arg_name.to_string()),
                        r#type: extract_type(ty)?,
                        default_value: None,
                        default_value_strategy: None,
                        docs: Docs::new(),
                    });
                }
            }
        }

        result.insert(
            name,
            InstructionNode {
                discriminators: vec![DiscriminatorNode::Constant(ConstantDiscriminatorNode::new(
                    ConstantValueNode::new(
                        NumberTypeNode::le(U8),
                        NumberValueNode::new(*dis as u8),
                    ),
                    0,
                ))],
                accounts,
                arguments,
                name: CamelCaseString::new(ix.name.to_string()),
                optional_account_strategy: InstructionOptionalAccountStrategy::ProgramId,
                ..Default::default()
            },
        );
    }

    Ok(result)
}

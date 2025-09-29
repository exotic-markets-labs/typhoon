use {
    crate::visitors::{
        get_type_node, ApplyInstructionVisitor, ProgramVisitor, SetAccountVisitor,
        SetDefinedTypesVisitor, SetErrorsVisitor, SetProgramIdVisitor,
    },
    codama::{
        CamelCaseString, CodamaResult, CombineModulesVisitor, ComposeVisitor,
        ConstantDiscriminatorNode, ConstantValueNode, DefinedTypeLinkNode, DiscriminatorNode, Docs,
        InstructionAccountNode, InstructionArgumentNode, InstructionNode,
        InstructionOptionalAccountStrategy, IsAccountSigner, NumberFormat::U8, NumberTypeNode,
        NumberValueNode, SetProgramMetadataVisitor, StructFieldTypeNode, StructTypeNode, TypeNode,
    },
    codama_korok_plugins::KorokPlugin,
    codama_korok_visitors::KorokVisitable,
    hashbrown::HashMap,
    syn::{Error, Type},
    typhoon_syn::{Arguments, InstructionArg},
};

pub struct TyphoonPlugin;

impl KorokPlugin for TyphoonPlugin {
    fn run(
        &self,
        visitable: &mut dyn KorokVisitable,
        next: &dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()>,
    ) -> CodamaResult<()> {
        next(visitable)?;

        let mut program_visitor = ProgramVisitor::new();
        visitable.accept(&mut program_visitor)?;

        let ixs = resolve_instructions(&program_visitor)?;

        let mut default_visitor = ComposeVisitor::new()
            .with(ApplyInstructionVisitor::new(ixs))
            .with(SetAccountVisitor::new())
            .with(SetDefinedTypesVisitor::new())
            .with(SetErrorsVisitor::new(program_visitor.errors_name))
            .with(SetProgramIdVisitor::new())
            .with(SetProgramMetadataVisitor::new())
            .with(CombineModulesVisitor::new());

        visitable.accept(&mut default_visitor)?;
        Ok(())
    }
}

fn resolve_instructions(
    program: &ProgramVisitor,
) -> CodamaResult<HashMap<String, InstructionNode>> {
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

fn extract_type(ty: &Type) -> Result<TypeNode, syn::Error> {
    if let Some(ty_node) = get_type_node(ty) {
        Ok(ty_node)
    } else {
        let Type::Path(ty_path) = ty else {
            return Err(Error::new_spanned(ty, "Invalid defined type."));
        };

        let seg = ty_path
            .path
            .segments
            .last()
            .ok_or(Error::new_spanned(ty, "Invalid defined path type."))?;

        Ok(TypeNode::Link(DefinedTypeLinkNode::new(
            seg.ident.to_string(),
        )))
    }
}

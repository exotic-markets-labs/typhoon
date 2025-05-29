use {
    codama::{CamelCaseString, Docs, InstructionAccountNode, IsAccountSigner, KorokVisitor, Node},
    codama_koroks::FieldKorok,
    syn::{GenericArgument, PathArguments, PathSegment, Type},
};

#[derive(Debug, Default)]
struct AccountFlags {
    is_signer: bool,
    is_mutable: bool,
    is_optional: bool,
}

pub struct ContextVisitor;

impl Default for ContextVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for ContextVisitor {
    fn visit_field(&mut self, korok: &mut FieldKorok) -> codama::CodamaResult<()> {
        let Some(name) = &korok.ast.ident else {
            return Ok(());
        };

        let mut flags = AccountFlags::default();
        extract_account_flags(&korok.ast.ty, &mut flags);

        let docs = crate::doc::Docs::from(korok.ast.attrs.as_slice());

        let node = InstructionAccountNode {
            name: CamelCaseString::new(name.to_string()),
            is_optional: flags.is_optional,
            is_signer: if flags.is_optional && flags.is_signer {
                IsAccountSigner::Either
            } else {
                flags.is_signer.into()
            },
            is_writable: flags.is_mutable,
            default_value: None,
            docs: Docs::from(docs.into_vec()),
        };
        korok.node = Some(Node::InstructionAccount(node));

        Ok(())
    }
}

fn extract_ty_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        Type::Path(ty_path) => ty_path.path.segments.last(),
        _ => None,
    }
}

fn extract_ty_from_arguments(args: &PathArguments) -> Option<&Type> {
    match args {
        PathArguments::AngleBracketed(generic_args) => {
            generic_args.args.first().and_then(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None,
            })
        }
        _ => None,
    }
}

fn extract_account_flags(ty: &Type, account_flags: &mut AccountFlags) {
    if let Some(PathSegment { ident, arguments }) = extract_ty_segment(ty) {
        let name = ident.to_string();
        match name.as_str() {
            "Option" => {
                if let Some(inner_ty) = extract_ty_from_arguments(arguments) {
                    account_flags.is_optional = true;
                    extract_account_flags(inner_ty, account_flags);
                }
            }
            "Mut" => {
                if let Some(inner_ty) = extract_ty_from_arguments(arguments) {
                    account_flags.is_mutable = true;
                    extract_account_flags(inner_ty, account_flags);
                }
            }
            "Signer" => account_flags.is_signer = true,
            _ => (),
        }
    }
}

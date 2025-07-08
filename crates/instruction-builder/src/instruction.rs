use {
    proc_macro2::Span,
    quote::format_ident,
    std::collections::HashMap,
    syn::{
        visit::{visit_path_segment, Visit},
        Attribute, FnArg, GenericArgument, Ident, Item, PathArguments, PathSegment, Type,
    },
    typhoon_syn::{
        arguments::Arguments,
        constraints::{Constraint, Constraints},
    },
};

const ACCOUNT_IDENTS: &[&str] = &[
    "Account",
    "BorshAccount",
    "InterfaceAccount",
    "Interface",
    "Program",
    "Signer",
    "SystemAccount",
    "UncheckedAccount",
];

#[derive(Default)]
pub struct Field {
    pub ty: String,
    pub is_account: bool,
    pub is_optional: bool,
    pub is_signer: bool,
    pub is_mutable: bool,
}

impl Visit<'_> for Field {
    fn visit_path_segment(&mut self, i: &syn::PathSegment) {
        let ty = i.ident.to_string();
        if ty == "Option" {
            self.is_optional = true;
        } else if ty == "Mut" {
            self.is_mutable = true;
        } else if ty == "Signer" {
            self.is_signer = true;
        }
        if !self.is_account {
            self.is_account = ACCOUNT_IDENTS.contains(&ty.as_str());
        }
        self.ty = ty;

        visit_path_segment(self, i);
    }
}

pub enum InstructionArg {
    Type(Type),
    Context((Ident, Arguments)),
}

pub struct Instruction {
    pub name: Ident,
    pub args: Vec<InstructionArg>,
    pub accounts: Vec<(Ident, (bool, bool, bool))>,
}

impl Instruction {
    fn rename_duplicate(&mut self) {
        let mut accounts: HashMap<String, u8> = HashMap::new();
        for (name, _) in self.accounts.iter_mut() {
            let name_str = name.to_string();
            let count = accounts.entry(name_str.clone()).or_insert(0);
            if *count > 0 {
                *name = format_ident!("{name_str}_{count}");
            }
            *count += 1;
        }
    }

    fn parse_arg(&mut self, seg: &PathSegment) -> syn::Result<()> {
        let PathArguments::AngleBracketed(args) = &seg.arguments else {
            return Err(syn::Error::new_spanned(seg, "Invalid argument."));
        };

        let arg_ty = args
            .args
            .iter()
            .find_map(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None,
            })
            .ok_or_else(|| syn::Error::new_spanned(seg, "Invalid argument type."))?;

        self.args.push(InstructionArg::Type(arg_ty.clone()));
        Ok(())
    }

    fn parse_context(&mut self, items: &[Item], seg: &PathSegment) -> syn::Result<()> {
        let context = items
            .iter()
            .find_map(|el| match el {
                Item::Struct(item_struct) if item_struct.ident == seg.ident => Some(item_struct),
                _ => None,
            })
            .ok_or(syn::Error::new_spanned(
                seg,
                "Cannot find the context struct.",
            ))?;
        if let Some(args) = context
            .attrs
            .iter()
            .find(|attr| attr.meta.path().is_ident("args"))
            .map(Arguments::try_from)
            .transpose()?
        {
            self.args
                .push(InstructionArg::Context((context.ident.clone(), args)));
        }

        for field in &context.fields {
            let mut ix_field = Field::default();
            ix_field.visit_field(field);

            let is_signer = is_signer(&field.attrs);

            if ix_field.is_account {
                self.accounts.push((
                    field
                        .ident
                        .as_ref()
                        .ok_or(syn::Error::new_spanned(field, "Invalid name"))
                        .cloned()?,
                    (
                        ix_field.is_optional,
                        ix_field.is_mutable,
                        is_signer || ix_field.is_signer,
                    ),
                ));
            } else {
                //TODO don't add bumps
                self.args.push(InstructionArg::Context((
                    context.ident.clone(),
                    Arguments::Struct(Ident::new(&ix_field.ty, Span::call_site())),
                )));
            }
        }
        Ok(())
    }

    pub fn parse_with_context(items: &[Item], name: &str) -> syn::Result<Self> {
        let ix_item = items
            .iter()
            .find_map(|i| match i {
                Item::Fn(item_fn) if item_fn.sig.ident == name => Some(item_fn),
                _ => None,
            })
            .ok_or(syn::Error::new(
                Span::call_site(),
                format!("Cannot find the instruction {name}"),
            ))?;

        let mut ix = Self {
            name: ix_item.sig.ident.clone(),
            accounts: Vec::new(),
            args: Vec::new(),
        };

        for fn_arg in ix_item.sig.inputs.iter() {
            let FnArg::Typed(pat_ty) = fn_arg else {
                continue;
            };

            let Type::Path(ref ty_path) = *pat_ty.ty else {
                continue;
            };

            let Some(seg) = ty_path.path.segments.last() else {
                continue;
            };

            if seg.ident == "ProgramIdArg" {
                continue;
            }

            if seg.ident == "Arg" || seg.ident == "BorshArg" {
                ix.parse_arg(seg)?;
            } else {
                ix.parse_context(items, seg)?;
            }
        }
        ix.rename_duplicate();

        Ok(ix)
    }
}

pub fn is_signer(attributes: &[Attribute]) -> bool {
    let Ok(constraints) = Constraints::try_from(attributes) else {
        return false;
    };

    let has_init = constraints
        .0
        .iter()
        .any(|c| matches!(c, Constraint::Init(_) | Constraint::InitIfNeeded(_)));
    let is_pda = constraints
        .0
        .iter()
        .any(|c| matches!(c, Constraint::Bump(_) | Constraint::AssociatedToken(_)));

    has_init && !is_pda
}

#[cfg(test)]
mod tests {
    use {super::*, syn::parse_quote};

    #[test]
    fn test_instruction_parse_with_context() {
        let mut meta = Field::default();
        let path: syn::Path = parse_quote!(Program<System>);
        meta.visit_path(&path);
        assert_eq!(meta.ty, "System");
        assert!(meta.is_account);
        assert!(!meta.is_mutable);
        assert!(!meta.is_optional);
        assert!(!meta.is_signer);

        let mut meta = Field::default();
        let path: syn::PathSegment = parse_quote!(Mut<Account<'info, Random>>);
        meta.visit_path_segment(&path);
        assert_eq!(meta.ty, "Random");
        assert!(meta.is_account);
        assert!(meta.is_mutable);
        assert!(!meta.is_optional);
        assert!(!meta.is_signer);

        let mut meta = Field::default();
        let field: syn::Field = parse_quote!(pub random2: Option<Account<'info, Random2>>);
        meta.visit_field(&field);
        assert_eq!(meta.ty, "Random2");
        assert!(meta.is_account);
        assert!(!meta.is_mutable);
        assert!(meta.is_optional);
        assert!(!meta.is_signer);

        let mut meta = Field::default();
        let field: syn::Field = parse_quote!(pub random2: RandomArgs);
        meta.visit_field(&field);
        assert_eq!(meta.ty, "RandomArgs");
        assert!(!meta.is_account);
        assert!(!meta.is_mutable);
        assert!(!meta.is_optional);
        assert!(!meta.is_signer);
    }
}

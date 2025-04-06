use {
    crate::{
        constraints::{Constraints, CONSTRAINT_IDENT_STR},
        generators::{
            BumpsGenerator, ConstraintGenerator, ConstraintGenerators, GeneratorResult,
            HasOneGenerator, InitializationGenerator, RentGenerator,
        },
        remover::AttributeRemover,
        visitor::ConstraintVisitor,
    },
    proc_macro2::Span,
    syn::{spanned::Spanned, visit_mut::VisitMut, Field, Ident, PathSegment, Type, TypePath},
};

#[derive(Clone)]
pub struct Account {
    pub(crate) name: Ident,
    pub(crate) constraints: Constraints,
    pub(crate) ty: PathSegment,
}

impl TryFrom<&mut Field> for Account {
    type Error = syn::Error;

    fn try_from(value: &mut Field) -> Result<Self, Self::Error> {
        let constraints = Constraints::try_from(&value.attrs)?;
        AttributeRemover::new(CONSTRAINT_IDENT_STR).visit_field_mut(value);

        let segment = match &value.ty {
            Type::Path(TypePath { path, .. }) => path.segments.last(),
            _ => None,
        }
        .ok_or_else(|| syn::Error::new(value.span(), "Invalid type for the account"))?;

        let name = value
            .ident
            .clone()
            .unwrap_or(Ident::new("random", Span::call_site())); //TODO unit type

        Ok(Account {
            name,
            constraints,
            ty: segment.clone(),
        })
    }
}
pub struct Accounts(pub Vec<Account>);

impl Accounts {
    // TODO avoid to do stupid things like this
    pub fn generate_tokens(&self, context_name: &Ident) -> syn::Result<GeneratorResult> {
        let mut generators = [
            ConstraintGenerators::Rent(RentGenerator::new()),
            ConstraintGenerators::Bumps(BumpsGenerator::new(context_name)),
            ConstraintGenerators::Init(InitializationGenerator::new()),
            ConstraintGenerators::HasOne(HasOneGenerator::new()),
        ];

        let mut result = GeneratorResult::default();
        for generator in &mut generators {
            generator.visit_accounts(self)?;
            let generated = generator.generate()?;

            if !generated.new_fields.is_empty() {
                result.new_fields.reserve(generated.new_fields.len());
                result.new_fields.extend(generated.new_fields);
            }

            if !generated.at_init.is_empty() {
                result.at_init.extend(generated.at_init);
            }
            if !generated.after_init.is_empty() {
                result.after_init.extend(generated.after_init);
            }
            if !generated.global_outside.is_empty() {
                result.global_outside.extend(generated.global_outside);
            }
        }

        Ok(result)
    }
}

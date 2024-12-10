use crate::ty::Type;

#[derive(Debug)]
pub enum TypeDef {
    Struct { fields: DefinedFields },
    Enum { variants: Vec<EnumVariant> },
    Type { alias: Type },
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Option<DefinedFields>,
}

#[derive(Debug)]
pub enum DefinedFields {
    Named(Vec<DefinedField>),
    Tuple(Vec<Type>),
    Unit,
}

// maybe add doc
#[derive(Debug)]
pub struct DefinedField {
    pub name: String,
    pub ty: Type,
}

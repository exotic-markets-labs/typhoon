use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Serialization {
    #[default]
    Borsh,
    Bytemuck,
    BytemuckUnsafe,
    Custom(String),
}

#[derive(Deserialize)]
pub struct ReprModifier {
    #[serde(default)]
    pub packed: bool,
    pub align: Option<usize>,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
#[non_exhaustive]
pub enum Repr {
    Rust(ReprModifier),
    C(ReprModifier),
    Transparent,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum DefinedFields {
    Named(Vec<Field>),
    Tuple(Vec<Type>),
}

#[derive(Deserialize, Clone)]
pub struct EnumVariant {
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub fields: Option<DefinedFields>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum TypeDefTy {
    Struct { fields: Option<DefinedFields> },
    Enum { variants: Vec<EnumVariant> },
    Type { alias: Type },
}

impl Default for TypeDefTy {
    fn default() -> Self {
        TypeDefTy::Struct { fields: None }
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum TypeDefGeneric {
    Type {
        name: String,
    },
    Const {
        name: String,
        #[serde(rename = "type")]
        ty: String,
    },
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ArrayLen {
    Generic(String),
    #[serde(untagged)]
    Value(usize),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    U128,
    I128,
    U256,
    I256,
    Bytes,
    String,
    #[serde(alias = "publicKey")]
    Pubkey,
    Option(Box<Type>),
    Vec(Box<Type>),
    Array(Box<Type>, ArrayLen),
    Defined(IdlDefined),
    Generic(String),
    #[serde(alias = "hashMap")]
    HashMap(Box<Type>, Box<Type>),
    #[serde(alias = "bTreeMap")]
    BTreeMap(Box<Type>, Box<Type>),
    #[serde(alias = "hashSet")]
    HashSet(Box<Type>),
    #[serde(alias = "bTreeSet")]
    BTreeSet(Box<Type>),
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum IdlDefined {
    Name(String),
    Object { name: String },
}

impl IdlDefined {
    pub fn name(&self) -> &str {
        match self {
            Self::Name(name) => name,
            Self::Object { name } => name,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct TypeDef {
    pub name: String,
    #[serde(default)]
    pub docs: Vec<String>,
    #[serde(default)]
    pub serialization: Serialization,
    pub repr: Option<Repr>,
    #[serde(default)]
    pub generics: Vec<TypeDefGeneric>,
    #[serde(rename = "type")]
    pub ty: TypeDefTy,
}

#[derive(Deserialize, Clone)]
pub struct Field {
    pub name: String,
    #[serde(default)]
    pub docs: Vec<String>,
    #[serde(rename = "type")]
    pub ty: Type,
}

#[derive(Deserialize)]
pub struct Instruction {
    pub name: String,
    #[serde(default)]
    pub docs: Vec<String>,
    pub discriminator: InstructionDiscriminator,
    pub accounts: Vec<InstructionAccountItem>,
    pub args: Vec<Field>,
    pub returns: Option<Type>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum InstructionDiscriminator {
    Anchor(Vec<u8>),
    Shank {
        #[serde(rename = "type")]
        ty: Type,
        value: u8,
    },
}

impl InstructionDiscriminator {
    pub fn value(&self) -> Vec<u8> {
        match self {
            InstructionDiscriminator::Anchor(items) => items.to_owned(),
            InstructionDiscriminator::Shank { value, .. } => vec![*value],
        }
    }
}

#[derive(Deserialize)]
pub struct InstructionAccount {
    pub name: String,

    #[serde(default)]
    #[serde(alias = "isMut")]
    #[serde(alias = "writable")]
    #[serde(alias = "is_mut")]
    #[serde(alias = "mutable")]
    pub is_mut: bool,

    #[serde(default)]
    #[serde(alias = "isSigner")]
    #[serde(alias = "signer")]
    #[serde(alias = "is_signer")]
    #[serde(alias = "signs")]
    pub is_signer: bool,
}

#[derive(Deserialize)]
pub struct InstructionAccounts {
    pub name: String,
    pub accounts: Vec<InstructionAccountItem>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum InstructionAccountItem {
    Composite(InstructionAccounts),
    Single(InstructionAccount),
}

#[derive(Deserialize)]
pub struct Metadata {
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize)]
pub struct Account {
    pub name: String,
    #[serde(default)]
    pub discriminator: Vec<u8>,
    #[serde(rename = "type")]
    pub ty: Option<TypeDefTy>,
}

#[derive(Deserialize)]
pub struct Idl {
    pub name: Option<String>,
    pub address: Option<String>,
    pub metadata: Metadata,
    pub accounts: Vec<Account>,
    pub instructions: Vec<Instruction>,
    #[serde(default)]
    pub types: Vec<TypeDef>,
}

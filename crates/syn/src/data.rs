use syn::{Ident, Type};

pub enum Encoding {
    Bytemuck,
    Borsh,
    Custom,
}

//TODO seeded
pub struct AccountData {
    pub name: Ident,
    pub encoding: Encoding,
    pub fields: Vec<(String, Type)>,
}

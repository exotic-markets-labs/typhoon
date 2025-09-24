use {
    crate::{account::InstructionAccount, arguments::Arguments},
    syn::{Ident, ItemStruct},
};

pub struct Context {
    pub name: Ident,
    pub accounts: Vec<InstructionAccount>,
    pub arguments: Option<Arguments>,
}

impl TryFrom<&ItemStruct> for Context {
    type Error = syn::Error;

    fn try_from(value: &ItemStruct) -> Result<Self, Self::Error> {
        let accounts = value
            .fields
            .iter()
            .map(InstructionAccount::try_from)
            .collect::<Result<_, _>>()?;

        let arguments = value
            .attrs
            .iter()
            .find(|attr| attr.meta.path().is_ident("args"))
            .and_then(|attr| Arguments::try_from(attr).ok());

        Ok(Context {
            name: value.ident.clone(),
            accounts,
            arguments,
        })
    }
}

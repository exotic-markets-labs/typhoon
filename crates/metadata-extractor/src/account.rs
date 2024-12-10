use {
    crate::{
        definition::{DefinedField, DefinedFields, TypeDef},
        ty::Type,
        Docs,
    },
    syn::ItemStruct,
};

#[derive(Debug)]
pub struct AccountState {
    pub name: String,
    pub docs: Docs,
    pub ty_def: TypeDef,
}

impl TryFrom<&ItemStruct> for AccountState {
    type Error = syn::Error;

    fn try_from(value: &ItemStruct) -> Result<Self, Self::Error> {
        let name = value.ident.to_string();
        let docs = Docs::from(value.attrs.as_slice());

        let ty_def = match &value.fields {
            syn::Fields::Named(fields_named) => {
                let fields = fields_named
                    .named
                    .iter()
                    .map(|field| {
                        let name = field.ident.as_ref().unwrap().to_string();
                        let ty = Type::try_from(&field.ty)?;
                        Ok(DefinedField { name, ty })
                    })
                    .collect::<Result<Vec<_>, syn::Error>>()?;
                TypeDef::Struct {
                    fields: DefinedFields::Named(fields),
                }
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                let types = fields_unnamed
                    .unnamed
                    .iter()
                    .map(|field| Type::try_from(&field.ty))
                    .collect::<Result<Vec<_>, syn::Error>>()?;
                TypeDef::Struct {
                    fields: DefinedFields::Tuple(types),
                }
            }
            syn::Fields::Unit => TypeDef::Struct {
                fields: DefinedFields::Unit,
            },
        };

        Ok(AccountState { name, docs, ty_def })
    }
}

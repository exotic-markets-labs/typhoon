use {
    crate::{injector::LifetimeInjector, remover::AttributeRemover},
    syn::{parse::Parse, spanned::Spanned, visit_mut::VisitMut, Item, ItemStruct},
    typhoon_syn::{constraints::CONSTRAINT_IDENT_STR, Account, Arguments},
};

pub struct ParsingContext {
    pub item_struct: ItemStruct,
    pub accounts: Vec<Account>,
    pub args: Option<Arguments>,
}

impl Parse for ParsingContext {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item: Item = input.parse()?;
        LifetimeInjector.visit_item_mut(&mut item);

        match item {
            Item::Struct(mut item_struct) => {
                let args = item_struct
                    .attrs
                    .iter()
                    .find(|attr| attr.meta.path().is_ident("args"))
                    .map(Arguments::try_from)
                    .transpose()?;

                let accounts: Vec<Account> = item_struct
                    .fields
                    .iter()
                    .map(Account::try_from)
                    .collect::<syn::Result<_>>()?;

                AttributeRemover::new("args").visit_item_struct_mut(&mut item_struct);
                AttributeRemover::new(CONSTRAINT_IDENT_STR).visit_item_struct_mut(&mut item_struct);

                Ok(ParsingContext {
                    item_struct,
                    accounts,
                    args,
                })
            }
            _ => Err(syn::Error::new(
                item.span(),
                "#[context] is only implemented for struct",
            )),
        }
    }
}

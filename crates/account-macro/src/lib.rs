use {
    keys::PrimaryKeys,
    quote::{quote, ToTokens},
    syn::{parse_macro_input, spanned::Spanned, Error, Item},
};

mod keys;

#[proc_macro_attribute]
pub fn account(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);
    let (name, generics, fields) = match item {
        Item::Struct(ref item_struct) => (
            &item_struct.ident,
            &item_struct.generics,
            &item_struct.fields,
        ),
        _ => {
            return Error::new(item.span(), "Invalid account type")
                .into_compile_error()
                .into()
        }
    };
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let keys = match PrimaryKeys::try_from(fields) {
        Ok(fields) => fields,
        Err(err) => return err.to_compile_error().into(),
    };
    let seeded_trait = keys.split_for_impl(name);

    let mut cleaned_item = item.clone();
    if let Item::Struct(item_struct) = &mut cleaned_item {
        for field in item_struct.fields.iter_mut() {
            field.attrs.retain(|attr| !attr.meta.path().is_ident("key"));
        }
    }

    quote! {
        #[derive(zerocopy::KnownLayout, zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::FromBytes)]
        #cleaned_item

        impl Owner for #name #ty_generics #where_clause {
            const OWNER: typhoon_program::pubkey::Pubkey = crate::ID;
        }

        impl Discriminator for #name #ty_generics #where_clause {
            const DISCRIMINATOR: &'static [u8] = &[];
        }

        #seeded_trait
    }
    .into_token_stream()
    .into()
}

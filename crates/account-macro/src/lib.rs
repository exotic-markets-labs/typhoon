use {
    keys::PrimaryKeys,
    quote::{quote, ToTokens},
    syn::{parse_macro_input, spanned::Spanned, Error, Item},
    typhoon_discriminator::DiscriminatorBuilder,
};

mod keys;

#[proc_macro_derive(AccountState, attributes(key, no_space))]
pub fn derive_account(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);
    let (attrs, name, generics, fields) = match item {
        Item::Struct(ref item_struct) => (
            &item_struct.attrs,
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

    let space_token = if attrs.iter().any(|a| a.path().is_ident("no_space")) {
        None
    } else {
        Some(quote! {
            impl #name {
                pub const SPACE: usize = <#name as Discriminator>::DISCRIMINATOR.len() + core::mem::size_of::<#name>();
            }
        })
    };
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let keys = match PrimaryKeys::try_from(fields) {
        Ok(fields) => fields,
        Err(err) => return err.to_compile_error().into(),
    };
    let seeded_trait = keys.split_for_impl(name);
    let discriminator = DiscriminatorBuilder::new(&name.to_string()).build();

    quote! {
        impl Owner for #name #ty_generics #where_clause {
            const OWNER: Pubkey = crate::ID;
        }

        impl Discriminator for #name #ty_generics #where_clause {
            const DISCRIMINATOR: &'static [u8] = &[#(#discriminator),*];
        }

        #space_token

        #seeded_trait
    }
    .into_token_stream()
    .into()
}

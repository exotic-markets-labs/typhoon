use {
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::{parse_macro_input, spanned::Spanned, Error, Item},
};

#[proc_macro_attribute]
pub fn account(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    let (name, generics) = match item {
        Item::Struct(ref item_struct) => (&item_struct.ident, &item_struct.generics),
        Item::Enum(ref item_enum) => (&item_enum.ident, &item_enum.generics),
        _ => {
            return Error::new(item.span(), "Invalid account type")
                .into_compile_error()
                .into()
        }
    };
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
        #[repr(C)]
        #item

        impl Owner for #name #ty_generics #where_clause {
            const OWNER: typhoon_program::pubkey::Pubkey = crate::ID;
        }

        impl Discriminator for #name #ty_generics #where_clause {
            const DISCRIMINATOR: &'static [u8] = &[];
        }
    }
    .into_token_stream()
    .into()
}

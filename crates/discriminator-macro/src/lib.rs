use {
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::{parse::Parse, parse_macro_input, LitStr},
    typhoon_discriminator::DiscriminatorBuilder,
};

#[proc_macro]
pub fn discriminator(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as Discriminator)
        .to_token_stream()
        .into()
}

struct Discriminator([u8; 8]);

impl Parse for Discriminator {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: LitStr = input.parse()?;

        let dis = DiscriminatorBuilder::new(&name.value()).build();

        Ok(Discriminator(dis))
    }
}

impl ToTokens for Discriminator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let discriminator = self.0;

        let t = quote!([#(#discriminator),*]);
        t.to_tokens(tokens);
    }
}

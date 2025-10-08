use {
    crate::{remover::AttributeRemover, size::borsh_size_gen, ty::SupportedType},
    proc_macro::TokenStream,
    proc_macro2::TokenStream as TokenStream2,
    quote::{format_ident, quote, ToTokens},
    syn::{fold::Fold, parse::Parse, parse_macro_input, Ident, Item},
};

mod remover;
#[cfg(feature = "test")]
mod replace;
mod size;
mod ty;

#[proc_macro_derive(BorshSize, attributes(max_len, raw_space))]
pub fn borsh_size_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    borsh_size_gen(&input).into()
}

#[proc_macro_attribute]
pub fn borsh(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ParsingContext);

    item.to_token_stream().into()
}

struct ParsingContext {
    raw_item: Item,
    name: Ident,
    fields: Vec<(Ident, SupportedType)>,
}

impl ParsingContext {
    fn calculate_last_offset(last_ident: &Option<(&Ident, TokenStream2)>) -> TokenStream2 {
        if let Some((n, expr)) = last_ident {
            let old_offset_name = format_ident!("{n}_offset");
            quote! {
                let mut offset = self.#old_offset_name();
                #expr
                offset
            }
        } else {
            quote!(0)
        }
    }
}

impl Parse for ParsingContext {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item: Item = input.parse()?;

        match item {
            Item::Enum(ref _item_enum) => {
                // item_enum.variants*
                todo!()

                /*
                   pub enum Repr(u8) {
                       Unit or full
                   }

                   impl

                */
            }
            Item::Struct(ref item_struct) => {
                let fields = item_struct
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let ident = field.ident.clone().unwrap_or(format_ident!("field_{i}"));
                        Ok((ident, (&field.ty).try_into()?))
                    })
                    .collect::<syn::Result<_>>()?;
                Ok(Self {
                    name: item_struct.ident.clone(),
                    raw_item: item,
                    fields,
                })
            }
            _ => unimplemented!("Only implemented for enum and struct"),
        }
    }
}

impl ToTokens for ParsingContext {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;

        let mut last_ident = None;
        let mut offsets = Vec::with_capacity(self.fields.len());
        let mut read_methods = Vec::with_capacity(self.fields.len());
        for (name, ty) in &self.fields {
            let last_offset = Self::calculate_last_offset(&last_ident);
            let offset_ident = format_ident!("{name}_offset");
            offsets.push(quote! {
                pub fn #offset_ident(&self) -> usize {
                    #last_offset
                }
            });
            let len_expr = ty.read_len_expr(name);
            last_ident = Some((name, len_expr));

            let (block, output_ty) = ty.gen_read_expr(false);
            let read_ident = format_ident!("{name}");
            if !block.is_empty() {
                read_methods.push(quote! {
                   pub fn #read_ident(&self) -> #output_ty {
                       let mut offset = self.#offset_ident();
                       #block
                   }
                });
            }
        }

        let last_offset = Self::calculate_last_offset(&last_ident);
        let len_token = borsh_size_gen(&self.raw_item);

        let raw_item = AttributeRemover::new()
            .with_attribute("raw_space")
            .with_attribute("max_len")
            .fold_item(self.raw_item.clone());

        #[cfg(feature = "test")]
        let expanded = {
            use {crate::replace::ReplaceName, syn::fold::Fold};

            let test_ident = format_ident!("{name}Test");
            let raw_item = ReplaceName(test_ident).fold_item(raw_item);
            quote! {
                #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
                #raw_item

                #[repr(transparent)]
                pub struct #name([u8]);

                impl #name {
                    #(#offsets)*

                    #(#read_methods)*

                    pub fn total_len(&self) -> usize {
                        #last_offset
                    }
                }

                #len_token
            }
        };

        #[cfg(not(feature = "test"))]
        let expanded = {
            let raw_item = &self.raw_item;

            quote! {
                #[cfg(not(target_os = "solana"))]
                #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
                #raw_item

                #[cfg(target_os = "solana")]
                #[repr(transparent)]
                pub struct #name([u8]);

                impl #name {
                    #(#offsets)*

                    #(#read_methods)*

                    pub fn total_len(&self) -> usize {
                        #last_offset
                    }
                }

                #len_token
            }
        };

        expanded.to_tokens(tokens);
    }
}

use {
    crate::{
        remover::AttributeRemover,
        replace::ReplaceName,
        size::{borsh_size_gen, borsh_size_gen_enum, borsh_size_gen_struct},
        ty::SupportedType,
    },
    proc_macro::TokenStream,
    proc_macro2::TokenStream as TokenStream2,
    quote::{format_ident, quote, ToTokens},
    syn::{
        fold::Fold, parse::Parse, parse_macro_input, parse_quote, FieldsNamed, FieldsUnnamed,
        Ident, Item, ItemEnum, ItemStruct,
    },
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

enum ParsingItem {
    Struct {
        item: ItemStruct,
        token: TokenStream2,
    },
    Enum {
        item: ItemEnum,
        variants: Vec<(Ident, Option<TokenStream2>)>,
    },
}

struct ParsingContext {
    item: ParsingItem,
    gen_size: bool,
}

impl Parse for ParsingContext {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item: Item = input.parse()?;

        match item {
            Item::Enum(item_enum) => {
                let variants = item_enum
                    .variants
                    .iter()
                    .map(|v| {
                        let token = match &v.fields {
                            syn::Fields::Named(FieldsNamed { named, .. }) => {
                                let name = format_ident!("{}Variant", v.ident);
                                let add_item = parse_quote! {
                                    pub struct #name {
                                        #named
                                    }
                                };
                                Some(generate_token(&add_item)?)
                            }
                            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. })
                                if unnamed.len() > 1 =>
                            {
                                let name = format_ident!("{}Variant", v.ident);
                                let add_item = parse_quote! {
                                    pub struct #name(#unnamed);
                                };
                                Some(generate_token(&add_item)?)
                            }
                            _ => None,
                        };
                        Ok((v.ident.clone(), token))
                    })
                    .collect::<syn::Result<_>>()?;

                Ok(Self {
                    gen_size: !item_enum
                        .attrs
                        .iter()
                        .any(|el| el.path().is_ident("no_size")),
                    item: ParsingItem::Enum {
                        item: item_enum,
                        variants,
                    },
                })
            }
            Item::Struct(item_struct) => Ok(Self {
                gen_size: !item_struct
                    .attrs
                    .iter()
                    .any(|el| el.path().is_ident("no_size")),
                item: ParsingItem::Struct {
                    token: generate_token(&item_struct)?,
                    item: item_struct,
                },
            }),
            _ => unimplemented!("Only implemented for enum and struct"),
        }
    }
}

impl ToTokens for ParsingContext {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expanded = match &self.item {
            ParsingItem::Struct { token, item } => {
                let borsh_size = self.gen_size.then(|| borsh_size_gen_struct(item));

                let original_item = AttributeRemover::new()
                    .with_attribute("max_len")
                    .with_attribute("raw_space")
                    .with_attribute("no_size")
                    .fold_item_struct(item.clone());

                let item = if cfg!(feature = "test") {
                    let item = ReplaceName(format_ident!("{}Test", original_item.ident))
                        .fold_item_struct(original_item);
                    quote!(#item)
                } else {
                    quote! {
                        #[cfg(not(target_os = "solana"))]
                        #original_item
                    }
                };

                quote! {
                    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
                    #item

                    #token

                    #borsh_size
                }
            }
            ParsingItem::Enum { item, variants } => {
                let name = &item.ident;
                let mut need_lifetime = false;
                let (variant_names, match_variants, add_structs): (
                    Vec<TokenStream2>,
                    Vec<TokenStream2>,
                    Vec<&Option<TokenStream2>>,
                ) = variants
                    .iter()
                    .map(|(var_name, add_struct)| {
                        if add_struct.is_some() {
                            let n = format_ident!("{var_name}Variant");
                            need_lifetime = true;
                            (
                                quote!(#var_name(&'a #n)),
                                quote!(#name::#var_name(item) => item.total_len(),),
                                add_struct,
                            )
                        } else {
                            (
                                quote!(#var_name),
                                quote!(#name::#var_name => 0,),
                                add_struct,
                            )
                        }
                    })
                    .collect();
                let mut new_item = item.clone();
                if need_lifetime {
                    new_item
                        .generics
                        .params
                        .push(syn::GenericParam::Lifetime(parse_quote!('a)));
                }

                let cfg_attr = if cfg!(feature = "test") {
                    None
                } else {
                    Some(quote! { #[cfg(target_os = "solana")] })
                };

                let raw_item = if cfg!(feature = "test") {
                    let item = ReplaceName(format_ident!("{}Test", item.ident))
                        .fold_item_enum(item.clone());
                    quote!(#item)
                } else {
                    quote! {
                        #[cfg(not(target_os = "solana"))]
                        #item
                    }
                };

                let (impl_generics, ty_generics, where_clause) = new_item.generics.split_for_impl();
                let borsh_size = self.gen_size.then(|| {
                    let size = borsh_size_gen_enum(&new_item);
                    quote! {
                        #cfg_attr
                        #size
                    }
                });

                quote! {
                    #(#add_structs)*

                    #raw_item

                    #cfg_attr
                    pub enum #name #ty_generics {
                        #(#variant_names),*
                    }

                    #cfg_attr
                    impl #impl_generics #name #ty_generics #where_clause {
                        pub fn total_len(&self) -> usize {
                            let len = match self {
                                #(#match_variants)*
                            };
                            1 + len
                        }
                    }

                    #borsh_size
                }
            }
        };
        expanded.to_tokens(tokens);
    }
}

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

fn generate_token(item_struct: &ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let fields: Vec<(Ident, SupportedType)> = item_struct
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field.ident.clone().unwrap_or(format_ident!("field_{i}"));
            Ok((ident, (&field.ty).try_into()?))
        })
        .collect::<syn::Result<_>>()?;
    let name = &item_struct.ident;

    let mut last_ident = None;
    let mut offsets = Vec::with_capacity(fields.len());
    let mut read_methods = Vec::with_capacity(fields.len());
    for (name, ty) in &fields {
        let last_offset = calculate_last_offset(&last_ident);
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

    let last_offset = calculate_last_offset(&last_ident);

    let cfg_attr = if cfg!(feature = "test") {
        None
    } else {
        Some(quote! { #[cfg(target_os = "solana")] })
    };

    let expanded = quote! {
        #cfg_attr
        #[repr(transparent)]
        pub struct #name([u8]);

        #cfg_attr
        impl #name {
            #(#offsets)*

            #(#read_methods)*

            pub fn total_len(&self) -> usize {
                #last_offset
            }
        }

        #cfg_attr
        impl<'a> BorshAccessor<'a> for &'a #name {
            #[inline(always)]
            fn len(data: &'a [u8]) -> usize {
                Self::convert(data).total_len()
            }

            #[inline(always)]
            fn convert(data: &'a [u8]) -> Self {
                unsafe { core::mem::transmute(data) }
            }
        }
    };

    Ok(expanded)
}

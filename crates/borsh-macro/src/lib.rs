use {
    crate::{remover::AttributeRemover, size::borsh_size_gen, ty::SupportedType},
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

// pub struct

enum ParsingContext {
    Struct {
        token: TokenStream2,
        size: TokenStream2,
    },
    Enum {
        raw_item: ItemEnum,
        variants: Vec<(Ident, Option<TokenStream2>)>,
        size: TokenStream2,
    },
}

impl Parse for ParsingContext {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item: Item = input.parse()?;

        match item {
            Item::Enum(ref item_enum) => {
                let mut inject_lifetime = false;
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
                                inject_lifetime = true;
                                Some(generate_token(&add_item)?)
                            }
                            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. })
                                if unnamed.len() > 1 =>
                            {
                                let name = format_ident!("{}Variant", v.ident);
                                let add_item = parse_quote! {
                                    pub struct #name(#unnamed);
                                };
                                inject_lifetime = true;
                                Some(generate_token(&add_item)?)
                            }
                            _ => None,
                        };
                        Ok((v.ident.clone(), token))
                    })
                    .collect::<syn::Result<_>>()?;

                let mut item_enum = item_enum.clone();
                if inject_lifetime {
                    item_enum
                        .generics
                        .params
                        .push(syn::GenericParam::Lifetime(parse_quote!('a)));
                }
                let size_token = borsh_size_gen(&Item::Enum(item_enum.clone()));

                let item_enum = AttributeRemover::new()
                    .with_attribute("raw_space")
                    .with_attribute("max_len")
                    .fold_item_enum(item_enum);

                Ok(Self::Enum {
                    raw_item: item_enum,
                    variants,
                    size: size_token,
                })
            }
            Item::Struct(ref item_struct) => {
                let size_token = borsh_size_gen(&item);

                let item = AttributeRemover::new()
                    .with_attribute("raw_space")
                    .with_attribute("max_len")
                    .fold_item_struct(item_struct.clone());

                Ok(Self::Struct {
                    token: generate_token(&item)?,
                    size: size_token,
                })
            }
            _ => unimplemented!("Only implemented for enum and struct"),
        }
    }
}

impl ToTokens for ParsingContext {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expanded = match self {
            ParsingContext::Struct { token, size } => {
                quote! {
                    #token

                    #size
                }
            }
            ParsingContext::Enum {
                raw_item,
                variants,
                size,
            } => {
                let name = &raw_item.ident;
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
                let (impl_generics, ty_generics, where_clause) = raw_item.generics.split_for_impl();

                //Variant name (item) => ItemName
                quote! {
                    #(#add_structs)*

                    pub enum #name #ty_generics {
                        #(#variant_names),*
                    }

                    impl #impl_generics #name #ty_generics #where_clause {
                        pub fn total_len(&self) -> usize {
                            let len = match self {
                                #(#match_variants)*
                            };
                            1 + len
                        }
                    }

                    #size
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

    #[cfg(feature = "test")]
    let expanded = {
        use {crate::replace::ReplaceName, syn::fold::Fold};

        let test_ident = format_ident!("{name}Test");
        let raw_item = ReplaceName(test_ident).fold_item_struct(item_struct.clone());
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

    Ok(expanded)
}

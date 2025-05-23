use {
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::{parse_macro_input, visit::Visit, Generics, Ident, Item, Type},
};

#[proc_macro_derive(ToMetas)]
pub fn derive_to_metas(item: proc_macro::TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);

    match item {
        Item::Struct(item_struct) => {
            let mut to_metas =
                ToMetas::new(item_struct.ident.clone(), item_struct.generics.clone());
            to_metas.visit_item_struct(&item_struct);
            to_metas.into_token_stream().into()
        }
        _ => unimplemented!(),
    }
}

struct ToMetas {
    name: Ident,
    generics: Generics,
    fields: Vec<Type>,
}

impl ToMetas {
    pub fn new(name: Ident, generics: Generics) -> Self {
        ToMetas {
            name,
            generics,
            fields: Vec::new(),
        }
    }
}

impl Visit<'_> for ToMetas {
    fn visit_field(&mut self, i: &syn::Field) {
        let Some(ident) = &i.ident else {
            return;
        };

        let ident_str = ident.to_string();
        if ident_str == "args" || ident_str == "bumps" {
            return;
        }

        self.fields.push(i.ty.clone());
    }
}

impl ToTokens for ToMetas {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let len = self.fields.len();
        let ty = &self.fields;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let expanded = quote! {
            impl #impl_generics ::typhoon_instruction_builder::ToMetas<#len> for #name #ty_generics #where_clause{
                fn to_metas() -> [(bool, bool, bool); #len] {
                    [
                        #(<#ty as Meta>::META,)*
                    ]
                }
            }
        };

        expanded.to_tokens(tokens);
    }
}

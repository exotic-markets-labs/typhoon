use {
    crate::ty::SupportedType,
    quote::format_ident,
    syn::{
        fold::{fold_item_struct, Fold},
        parse_quote, Ident,
    },
};

pub struct ReplaceName(pub Ident);

impl Fold for ReplaceName {
    fn fold_item_struct(&mut self, mut i: syn::ItemStruct) -> syn::ItemStruct {
        i.ident = self.0.to_owned();
        fold_item_struct(self, i)
    }

    fn fold_type(&mut self, i: syn::Type) -> syn::Type {
        let Ok(ty) = (&i).try_into() else {
            return i;
        };

        if let SupportedType::Defined(name) = ty {
            let ident = format_ident!("{name}Test");
            parse_quote!(#ident)
        } else {
            i
        }
    }
}

mod arg;
mod value;

use {
    crate::size::arg::{Arg, Args},
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        parse_quote, AngleBracketedGenericArguments, Expr, Field, Fields, GenericParam, Generics,
        Item, ItemEnum, ItemStruct, PathArguments, Type, TypeArray, TypePath, Variant,
    },
};

pub fn borsh_size_gen_struct(
    ItemStruct {
        ref ident,
        generics,
        fields,
        ..
    }: &ItemStruct,
) -> TokenStream {
    let generics = add_trait_bounds(generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let len_expr = get_len_expr_from_fields(fields);

    quote! {
        impl #impl_generics BorshSize for #ident #ty_generics #where_clause {
            const SIZE: usize = #len_expr;
        }
    }
}

pub fn borsh_size_gen_enum(
    ItemEnum {
        ref ident,
        variants,
        generics,
        ..
    }: &ItemEnum,
) -> TokenStream {
    let variants = variants
        .into_iter()
        .map(|Variant { fields, .. }| get_len_expr_from_fields(fields));
    let generics = add_trait_bounds(generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let max = gen_max(variants);

    quote! {
        impl #impl_generics BorshSize for #ident #ty_generics #where_clause {
            const SIZE: usize = 1 + #max;
        }
    }
}

pub fn borsh_size_gen(input: &Item) -> TokenStream {
    match input {
        Item::Enum(item) => borsh_size_gen_enum(item),
        Item::Struct(item) => borsh_size_gen_struct(item),
        _ => unimplemented!("Borsh size is only implemented on struct and enum."),
    }
}

// Add a bound `T: Space` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(BorshSize));
        }
    }
    generics
}

fn gen_max<T: Iterator<Item = TokenStream>>(mut iter: T) -> TokenStream {
    if let Some(item) = iter.next() {
        let next_item = gen_max(iter);
        quote!(max(#item, #next_item))
    } else {
        quote!(0)
    }
}

fn get_len_expr_from_fields(fields: &Fields) -> TokenStream {
    let len = fields.into_iter().map(|f| match TyLen::try_from(f) {
        Ok(TyLen(len)) => quote!(#len),
        Err(err) => err.into_compile_error(),
    });

    quote!(0 #(+ #len)*)
}

fn expr_from_ty(value: &Type, args: &mut Vec<Arg>) -> syn::Result<Expr> {
    let current_arg = args.pop();

    let arg = match current_arg {
        Some(Arg { is_raw, ref value }) => {
            if is_raw {
                return Ok(parse_quote!(#value));
            } else {
                Some(value)
            }
        }
        None => None,
    };

    match value {
        Type::Array(TypeArray { elem, len, .. }) => {
            let inner_ty = expr_from_ty(elem, args)?;

            Ok(parse_quote!((#len * #inner_ty)))
        }
        Type::Path(TypePath { ref path, .. }) => {
            let Some(segment) = path.segments.last() else {
                return Err(syn::Error::new_spanned(value, "Invalid path type."));
            };
            let ident = &segment.ident;

            match ident.to_string().as_str() {
                "String" => {
                    let Some(arg_value) = arg else {
                        return Err(syn::Error::new_spanned(ident, "No max_len specified."));
                    };

                    Ok(parse_quote!((4 + #arg_value)))
                }
                "Vec" => {
                    let Some(arg_value) = arg else {
                        return Err(syn::Error::new_spanned(ident, "No max_len specified."));
                    };

                    let new_ty = parse_first_arg(&segment.arguments)?;
                    let new_len = expr_from_ty(&new_ty, args)?;

                    Ok(parse_quote!((4 + #new_len * #arg_value)))
                }
                _ => Ok(parse_quote!(<#value as BorshSize>::SIZE)),
            }
        }
        _ => Ok(parse_quote!(<#value as BorshSize>::SIZE)),
    }
}

fn parse_first_arg(path_args: &PathArguments) -> syn::Result<Type> {
    let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = path_args
    else {
        return Err(syn::Error::new_spanned(
            path_args,
            "Invalid type of arguments.",
        ));
    };

    match &args[0] {
        syn::GenericArgument::Type(ty) => Ok(ty.to_owned()),
        _ => Err(syn::Error::new_spanned(
            path_args,
            "The first argument is not a type.",
        )),
    }
}

struct TyLen(Expr);

impl TryFrom<&Field> for TyLen {
    type Error = syn::Error;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        let Some(ref name) = value.ident else {
            return Err(syn::Error::new_spanned(
                value,
                "Tuple field is not allowed.",
            ));
        };

        let mut attr_args = value.attrs.iter().filter_map(|a| Args::try_from(a).ok());

        let args = attr_args.by_ref().take(1).next();

        if attr_args.next().is_some() {
            return Err(syn::Error::new(
                name.span(),
                "max_len and raw_space cannot be used at the same time.",
            ));
        }

        let expr = expr_from_ty(&value.ty, &mut args.unwrap_or_default())?;

        Ok(TyLen(expr))
    }
}

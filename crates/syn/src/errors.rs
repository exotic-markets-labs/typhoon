use syn::{
    parse::Parse, punctuated::Punctuated, token::Comma, Attribute, Data, DeriveInput, Expr,
    ExprLit, Ident, ItemEnum, Lit, LitStr, Variant,
};

fn parse_attribute(attributes: &[Attribute]) -> Option<String> {
    attributes.iter().find_map(|attr| {
        if !attr.path().is_ident("msg") {
            return None;
        }

        let lit: LitStr = attr.parse_args().ok()?;
        Some(lit.value())
    })
}

pub struct ErrorVariant {
    pub discriminant: u32,
    pub name: Ident,
    pub msg: String,
}

pub struct Errors {
    pub name: Ident,
    pub variants: Vec<ErrorVariant>,
}

impl TryFrom<&ItemEnum> for Errors {
    type Error = syn::Error;

    fn try_from(value: &ItemEnum) -> Result<Self, Self::Error> {
        Ok(Errors {
            name: value.ident.clone(),
            variants: parse_variants(&value.variants)?,
        })
    }
}

impl Parse for Errors {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;

        let Data::Enum(data) = &derive_input.data else {
            return Err(syn::Error::new_spanned(
                &derive_input,
                "TyphoonError can only be used on enums",
            ));
        };

        Ok(Errors {
            name: derive_input.ident,
            variants: parse_variants(&data.variants)?,
        })
    }
}

fn parse_variants(data_variants: &Punctuated<Variant, Comma>) -> syn::Result<Vec<ErrorVariant>> {
    let mut variants = Vec::with_capacity(data_variants.len());
    let mut latest_dis: isize = -1;

    for variant in data_variants {
        let variant_name = &variant.ident;
        let msg = parse_attribute(&variant.attrs)
            .ok_or(syn::Error::new_spanned(variant, "No error msg set."))?;

        if let Some((_, ref expr)) = variant.discriminant {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(val), ..
            }) = expr
            {
                latest_dis = val.base10_parse::<isize>()?
            } else {
                return Err(syn::Error::new_spanned(expr, "Invalid discriminant."));
            }
        } else {
            latest_dis += 1;
        }

        variants.push(ErrorVariant {
            name: variant_name.to_owned(),
            msg,
            discriminant: latest_dis as u32,
        });
    }

    Ok(variants)
}

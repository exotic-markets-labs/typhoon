use {
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::{
        parse::Parse, parse_macro_input, Attribute, Data, DeriveInput, Expr, ExprLit, Ident, Lit,
        LitStr,
    },
};

/// Derive macro for generating error implementations
///
/// Usage:
/// ```rust
/// # use {
/// #    pinocchio::program_error::{ProgramError, ToStr},
/// #    typhoon_errors::Error,
/// #    typhoon_errors_macro::TyphoonError
/// # };
/// #[derive(TyphoonError)]
/// pub enum MyError {
///     #[msg("Error: Invalid owner")]
///     InvalidOwner = 200,
///     #[msg("Error: Insufficient funds")]
///     InsufficientFunds,
/// }
/// ```
#[proc_macro_derive(TyphoonError, attributes(msg))]
pub fn typhoon_error(input: TokenStream) -> TokenStream {
    let errors_token = parse_macro_input!(input as Errors);

    errors_token.to_token_stream().into()
}

fn parse_attribute(attributes: &[Attribute]) -> Option<String> {
    attributes.iter().find_map(|attr| {
        if !attr.path().is_ident("msg") {
            return None;
        }

        let lit: LitStr = attr.parse_args().ok()?;
        Some(lit.value())
    })
}

struct Variant {
    discriminant: u32,
    name: Ident,
    msg: String,
}

struct Errors {
    name: Ident,
    variants: Vec<Variant>,
}

impl Parse for Errors {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;

        let Data::Enum(data) = &derive_input.data else {
            return Err(syn::Error::new_spanned(
                &derive_input,
                "TyphoonDerive can only be used on enums",
            ));
        };

        let mut variants = Vec::with_capacity(data.variants.len());
        let mut latest_dis: isize = -1;

        for variant in &data.variants {
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

            variants.push(Variant {
                name: variant_name.to_owned(),
                msg,
                discriminant: latest_dis as u32,
            });
        }

        Ok(Errors {
            name: derive_input.ident,
            variants,
        })
    }
}

impl ToTokens for Errors {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;

        let (to_str_arms, try_from_arms) = self
            .variants
            .iter()
            .map(|v| {
                let variant_name = &v.name;
                let msg = &v.msg;
                let discriminant = &v.discriminant;
                (
                    quote!(#name::#variant_name => #msg,),
                    quote!(#discriminant => Ok(#name::#variant_name),),
                )
            })
            .collect::<(Vec<_>, Vec<_>)>();

        quote! {
            impl TryFrom<u32> for #name {
                type Error = ProgramError;

                fn try_from(value: u32) -> Result<Self, Self::Error> {
                    match value {
                        #(#try_from_arms)*
                        _ => Err(ProgramError::InvalidArgument),
                    }
                }
            }

            impl ToStr for #name {
                fn to_str<E>(&self) -> &'static str
                where
                    E: 'static + ToStr + TryFrom<u32>,
                {
                    match self {
                        #(#to_str_arms)*
                    }
                }
            }

            impl From<#name> for Error {
                fn from(value: #name) -> Self {
                    Error::new(ProgramError::Custom(value as u32))
                }
            }
        }
        .to_tokens(tokens);
    }
}

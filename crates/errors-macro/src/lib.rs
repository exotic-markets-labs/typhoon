use {
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::parse_macro_input,
    typhoon_syn::Errors,
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

    let name = &errors_token.name;

    let (to_str_arms, try_from_arms) = errors_token
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
    .into_token_stream()
    .into()
}

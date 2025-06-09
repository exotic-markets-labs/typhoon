mod client;
mod cpi;

use {
    crate::instruction::{Instruction, InstructionArg},
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
    typhoon_syn::arguments::{Argument, Arguments},
};
pub use {client::*, cpi::*};

pub trait Generator {
    fn generate_token(ix: &[(usize, Instruction)]) -> TokenStream;
}

pub fn generate_argument(arg: &InstructionArg) -> (TokenStream, Option<(String, TokenStream)>) {
    match arg {
        InstructionArg::Type(ty) => (quote!(#ty), None),
        InstructionArg::Context((context_name, args)) => match args {
            Arguments::Struct(ident) => (ident.to_token_stream(), None),
            Arguments::Values(args) => {
                let struct_name = format_ident!("{context_name}Args");
                let name_str = struct_name.to_string();
                let fields = args
                    .iter()
                    .map(|Argument { name, ty }: &Argument| quote!(pub #name: #ty));
                let item = quote! {
                    #[derive(Debug, PartialEq, bytemuck::AnyBitPattern, bytemuck::NoUninit, Copy, Clone)]
                    #[repr(C)]
                    pub struct #struct_name {
                        #(#fields),*
                    }
                };

                (
                    format_ident!("{context_name}Args").to_token_stream(),
                    Some((name_str, item)),
                )
            }
        },
    }
}

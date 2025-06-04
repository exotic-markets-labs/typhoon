mod client;
// mod cpi;

pub use client::*;
use {crate::instruction::Instruction, proc_macro2::TokenStream};

pub trait Generator {
    fn generate_token(ix: &[(usize, Instruction)]) -> TokenStream;
}

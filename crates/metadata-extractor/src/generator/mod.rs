mod client;
mod cpi;

use {crate::instruction::Instruction, proc_macro2::TokenStream};
pub use {client::*, cpi::*};

pub trait Generator {
    fn generate_token(ix: &[(usize, Instruction)]) -> TokenStream;
}

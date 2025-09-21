mod client;
mod cpi;

pub use {client::*, cpi::*};
use {
    hashbrown::HashMap,
    proc_macro2::TokenStream,
    typhoon_syn::{Context, Instruction},
};

pub trait Generator {
    fn generate_token(
        instructions: &HashMap<usize, Instruction>,
        context: &HashMap<String, Context>,
        extra_token: TokenStream,
    ) -> TokenStream;
}

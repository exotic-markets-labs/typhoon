use std::{fs::read_to_string, path::Path};

use anchor_lang_idl_spec::Idl;
use proc_macro::TokenStream;
use syn::{parse::Parse, parse_macro_input, LitStr};
use typhoon_cpi_generator::anchor::gen_cpi;

#[proc_macro]
pub fn anchor_cpi(input: TokenStream) -> TokenStream {
    let idl_file = parse_macro_input!(input as IdlFile);
    let idl: Idl = serde_json::from_str(&idl_file.content).unwrap();

    gen_cpi(&idl).into()
}
struct IdlFile {
    pub content: String,
}

impl Parse for IdlFile {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: LitStr = input.parse()?;
        let path_str = path.value();

        let content = read_to_string(&path_str)
            .map_err(|_| syn::Error::new(path.span(), "Unable to read file"))?;

        Ok(IdlFile { content })
    }
}

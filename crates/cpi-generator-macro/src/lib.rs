use {
    anchor_lang_idl_spec::Idl,
    proc_macro::TokenStream,
    std::{fs::read_to_string, path::PathBuf},
    syn::{parse::Parse, parse_macro_input, LitStr},
    typhoon_cpi_generator::anchor::gen_cpi,
};

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

        let var = std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|err| syn::Error::new(input.span(), err.to_string()))?;
        let mut so_path = PathBuf::from(var);
        so_path.push(path_str);

        let content = read_to_string(so_path)
            .map_err(|_| syn::Error::new(path.span(), "Unable to read file"))?;

        Ok(IdlFile { content })
    }
}

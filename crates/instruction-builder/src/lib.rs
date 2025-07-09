use {
    crate::{
        generator::{ClientGenerator, CpiGenerator, Generator},
        instruction::Instruction,
        resolver::Resolver,
    },
    cargo_manifest::{Dependency, Manifest},
    heck::ToKebabCase,
    proc_macro2::{Span, TokenStream},
    quote::ToTokens,
    std::path::Path,
    syn::{
        parse::{Parse, Parser},
        parse_macro_input,
        punctuated::Punctuated,
        visit::Visit,
        Ident, Token,
    },
};

mod generator;
mod instruction;
mod mod_path;
mod resolver;

#[proc_macro]
pub fn generate_instructions_client(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let instructions = parse_macro_input!(input as Instructions);

    instructions
        .generate::<ClientGenerator>()
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn generate_cpi_client(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let instructions = parse_macro_input!(input as Instructions);

    instructions
        .generate::<CpiGenerator>()
        .into_token_stream()
        .into()
}

#[derive(Default)]
struct InstructionsList(Vec<(usize, String)>);

impl Visit<'_> for InstructionsList {
    fn visit_item_macro(&mut self, i: &syn::ItemMacro) {
        if !i.mac.path.is_ident("handlers") {
            return;
        }

        if let Ok(instructions) =
            Punctuated::<Ident, syn::Token![,]>::parse_terminated.parse2(i.mac.tokens.clone())
        {
            self.0 = instructions
                .iter()
                .enumerate()
                .map(|(i, n)| (i, n.to_string()))
                .collect()
        };
    }
}

impl From<InstructionsList> for Vec<(usize, String)> {
    fn from(value: InstructionsList) -> Self {
        value.0
    }
}

struct Instructions {
    ix: Vec<(usize, Instruction)>,
}

impl Instructions {
    pub fn generate<T: Generator>(&self) -> TokenStream {
        T::generate_token(&self.ix)
    }
}

impl Parse for Instructions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let krate: Ident = input.parse()?;
        let crate_name = krate.to_string();
        let crate_kebab_name = crate_name.to_kebab_case();
        let cargo_toml_dir = std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|_| syn::Error::new(Span::call_site(), "Not in valid rust project."))?;
        // let temp_dir = env!("PROC_ARTIFACT_DIR");
        let manifest = Manifest::from_path(format!("{cargo_toml_dir}/Cargo.toml"))
            .map_err(|_| syn::Error::new(Span::call_site(), "Invalid Cargo.toml file."))?;

        let package_relative_path = get_package_path(&manifest, &crate_kebab_name).ok_or(
            syn::Error::new(Span::call_site(), "Cannot find the package."),
        )?;

        let package_absolute_path = format!("{cargo_toml_dir}/{package_relative_path}");
        let path = Path::new(&package_absolute_path);
        let file = read_and_parse_file(path)?;

        let mut ix_list = InstructionsList::default();
        ix_list.visit_file(&file);
        let ix = ix_list.0.iter();

        let mut resolver = Resolver::new(path, true);
        resolver.visit_file(&file);

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let content;
            let _ = syn::bracketed!(content in input);
            let instructions = content
                .parse_terminated(Ident::parse, Token![,])?
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>();

            Ok(Self {
                ix: ix
                    .filter(|(_, n)| instructions.contains(n))
                    .map(|(i, n)| Ok((*i, resolver.get_instruction(n)?)))
                    .collect::<syn::Result<_>>()?,
            })
        } else {
            Ok(Self {
                ix: ix
                    .map(|(i, n)| Ok((*i, resolver.get_instruction(n)?)))
                    .collect::<syn::Result<_>>()?,
            })
        }
    }
}

fn read_and_parse_file(source_file: impl AsRef<Path>) -> syn::Result<syn::File> {
    let file_content = std::fs::read_to_string(&source_file)
        .map_err(|err| syn::Error::new(Span::call_site(), err.to_string()))?;

    syn::parse_file(&file_content)
}

fn get_package_path(manifest: &Manifest, crate_name: &str) -> Option<String> {
    let package = manifest.package.as_ref()?;
    if package.name.to_kebab_case() == crate_name {
        Some("src/lib.rs".to_string())
    } else {
        let dependency: Dependency = package
            .metadata
            .as_ref()?
            .get("typhoon")?
            .as_table()?
            .get("builder-dependencies")?
            .as_table()?
            .iter()
            .find_map(|(key, value)| (key.to_kebab_case() == crate_name).then_some(value))?
            .clone()
            .try_into()
            .ok()?;
        Some(format!(
            "{}/src/lib.rs",
            dependency.detail()?.path.as_ref()?
        ))
    }
}

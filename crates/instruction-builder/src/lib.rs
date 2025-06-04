use {
    crate::{
        generator::{ClientGenerator, Generator},
        instruction::Instruction,
        resolver::Resolver,
    },
    cargo_metadata::MetadataCommand,
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

        let metadata = MetadataCommand::new()
            .exec()
            .map_err(|_| syn::Error::new_spanned(&crate_name, "Failed to get cargo metadata"))?;
        let source_file = metadata
            .packages
            .iter()
            .find(|p| crate_name == *p.name || crate_kebab_name == *p.name)
            .and_then(|p| p.targets.first())
            .map(|t| t.src_path.to_string())
            .ok_or(syn::Error::new_spanned(
                &crate_name,
                "Could not find source file for package",
            ))?;
        let path = Path::new(&source_file);
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

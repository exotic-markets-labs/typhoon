use {
    crate::{
        generator::{ClientGenerator, CpiGenerator, Generator},
        resolver::Resolver,
    },
    cargo_manifest::{Dependency, Manifest},
    hashbrown::{HashMap, HashSet},
    heck::ToKebabCase,
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote, ToTokens},
    std::path::Path,
    syn::{
        parse::{Parse, Parser},
        parse_macro_input,
        punctuated::Punctuated,
        visit::Visit,
        Ident, Item, Token,
    },
    typhoon_syn::{Argument, Arguments, Context, Instruction, InstructionArg},
};

mod generator;
mod mod_path;
mod resolver;

#[proc_macro]
pub fn generate_instructions_client(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let instructions = parse_macro_input!(input as GeneratorContext);

    instructions
        .generate::<ClientGenerator>()
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn generate_cpi_client(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let instructions = parse_macro_input!(input as GeneratorContext);

    instructions
        .generate::<CpiGenerator>()
        .into_token_stream()
        .into()
}

#[derive(Default)]
struct InstructionsList(pub Vec<(usize, Ident)>);

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
                .map(|(i, n)| (i, n.clone()))
                .collect()
        };
    }
}

impl From<InstructionsList> for Vec<(usize, Ident)> {
    fn from(value: InstructionsList) -> Self {
        value.0
    }
}

#[derive(Default)]
struct GeneratorContext {
    pub instructions: HashMap<usize, Instruction>,
    pub context: HashMap<String, Context>,
    pub arg_structs: HashMap<String, Vec<Argument>>,
}

impl GeneratorContext {
    pub fn from_resolver(
        ix_list: InstructionsList,
        resolver: Resolver,
        filter: Option<HashSet<Ident>>,
    ) -> Self {
        let mut gen = GeneratorContext::default();
        let (mut instructions_map, mut contexts_map) = Self::parse_items(&resolver.items);

        for (index, ident) in ix_list.0 {
            if let Some(ref filter_set) = filter {
                if !filter_set.contains(&ident) {
                    continue;
                }
            }

            if let Some(instruction) = instructions_map.remove(&ident) {
                gen.instructions.insert(index, instruction);
            }
        }

        for ix in gen.instructions.values() {
            for (_, arg_value) in &ix.args {
                if let InstructionArg::Context(ctx_name) = arg_value {
                    let ctx_name = ctx_name.to_string();
                    if let Some(context) = contexts_map.remove(&ctx_name) {
                        if let Some(Arguments::Values(ref args)) = context.arguments {
                            gen.arg_structs
                                .entry(format!("{}Args", context.name))
                                .or_insert_with(|| args.to_vec());
                        }

                        gen.context.insert(ctx_name, context);
                    }
                }
            }
        }
        gen
    }

    fn parse_items(items: &[Item]) -> (HashMap<Ident, Instruction>, HashMap<String, Context>) {
        let mut instructions = HashMap::new();
        let mut contexts = HashMap::new();

        for item in items {
            match item {
                Item::Fn(item_fn) => {
                    if let Ok(ix) = Instruction::try_from(item_fn) {
                        instructions.insert(ix.name.clone(), ix);
                    }
                }
                Item::Struct(item_struct) => {
                    if let Ok(ctx) = Context::try_from(item_struct) {
                        contexts.insert(ctx.name.to_string(), ctx);
                    }
                }
                _ => continue,
            }
        }

        (instructions, contexts)
    }

    pub fn generate<T: Generator>(&self) -> TokenStream {
        let extra_token: Vec<TokenStream> = self.arg_structs.iter().map(|(name, v)| {
            let struct_name = format_ident!("{name}");
            let fields = v
            .iter()
            .map(|Argument { name, ty }: &Argument| quote!(pub #name: #ty));
       quote! {
            #[derive(Debug, PartialEq, bytemuck::AnyBitPattern, bytemuck::NoUninit, Copy, Clone)]
            #[repr(C)]
            pub struct #struct_name {
                #(#fields),*
            }
        }
        }).collect();
        T::generate_token(&self.instructions, &self.context, quote!(#(#extra_token)*))
    }
}

impl Parse for GeneratorContext {
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

        let mut resolver = Resolver::new(path, true);
        resolver.visit_file(&file);

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let content;
            let _ = syn::bracketed!(content in input);
            let idents = content.parse_terminated(Ident::parse, Token![,])?;
            let instructions = HashSet::from_iter(idents);
            Ok(Self::from_resolver(ix_list, resolver, Some(instructions)))
        } else {
            Ok(Self::from_resolver(ix_list, resolver, None))
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

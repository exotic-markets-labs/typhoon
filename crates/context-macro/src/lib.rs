use {
    crate::cross_checks::cross_checks,
    context::Context,
    generators::*,
    injector::FieldInjector,
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::{parse_macro_input, parse_quote, visit_mut::VisitMut, Attribute, Ident, Lifetime},
};

mod accounts;
mod arguments;
mod constraints;
mod context;
mod cross_checks;
mod extractor;
mod generators;
mod injector;
mod remover;
mod visitor;

#[proc_macro_attribute]
pub fn context(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let context = parse_macro_input!(item as Context);
    let generator = match TokenGenerator::new(context) {
        Ok(gen) => gen,
        Err(err) => return TokenStream::from(err.into_compile_error()),
    };

    TokenStream::from(generator.into_token_stream())
}

struct TokenGenerator {
    context: Context,
    result: GeneratorResult,
}

trait StagedGenerator {
    fn append(&mut self, context: &mut GenerationContext) -> Result<(), syn::Error>;
}

struct GenerationContext {
    input: Context,
    generated_results: GeneratorResult,
}

impl TokenGenerator {
    pub fn new(context: Context) -> Result<Self, syn::Error> {
        let mut generation_context = GenerationContext {
            input: context,
            generated_results: GeneratorResult::default(),
        };
        let mut generators = [
            ConstraintGenerators::Args(ArgumentsGenerator::new()),
            ConstraintGenerators::Assign(AssignGenerator::new()),
            ConstraintGenerators::Rent(RentGenerator::new()),
            ConstraintGenerators::Bumps(BumpsGenerator::new()),
            ConstraintGenerators::HasOne(HasOneGenerator::new()),
            ConstraintGenerators::Token(TokenAccountGenerator),
        ];

        cross_checks(&generation_context)?;

        for generator in &mut generators {
            generator.append(&mut generation_context)?;
        }

        Ok(TokenGenerator {
            context: generation_context.input,
            result: generation_context.generated_results,
        })
    }
}

impl ToTokens for TokenGenerator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.context.item_struct.ident;
        let generics = &self.context.item_struct.generics;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let new_lifetime: Lifetime = parse_quote!('info);

        let outside = &self.result.outside;
        let inside = &self.result.inside;

        let name_list: Vec<&Ident> = self
            .context
            .item_struct
            .fields
            .iter()
            .filter_map(|f| f.ident.as_ref())
            .collect();

        let mut struct_fields: Vec<&Ident> = name_list.clone();

        let account_struct = &mut self.context.item_struct.to_owned();
        for new_field in &self.result.new_fields {
            FieldInjector::new(new_field.clone()).visit_item_struct_mut(account_struct);

            struct_fields.push(new_field.ident.as_ref().unwrap());
        }

        let impl_context = quote! {
            impl #impl_generics HandlerContext<#new_lifetime> for #name #ty_generics #where_clause {
                fn from_entrypoint(
                    accounts: &mut &'info [AccountInfo],
                    instruction_data: &mut &'info [u8],
                ) -> Result<Self, ProgramError> {
                    let [#(#name_list,)* rem @ ..] = accounts else {
                        return Err(ProgramError::NotEnoughAccountKeys);
                    };

                    #inside

                    *accounts = rem;

                    Ok(#name { #(#struct_fields),* })
                }
            }
        };

        let doc = prettyplease::unparse(
            &syn::parse2::<syn::File>(quote! {
                #outside

                #impl_context
            })
            .unwrap(),
        );

        let mut doc_attrs: Vec<Attribute> = parse_quote! {
            /// # Generated
            /// ```ignore
            #[doc = #doc]
            /// ```
        };

        account_struct.attrs.append(&mut doc_attrs);

        let expanded = quote! {
            #outside

            #account_struct

            #impl_context

        };
        expanded.to_tokens(tokens);
    }
}

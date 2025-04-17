use {
    context::Context,
    generators::*,
    injector::FieldInjector,
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::{parse_macro_input, parse_quote, visit_mut::VisitMut, Attribute, Ident, Lifetime},
    visitor::ContextVisitor,
};

mod accounts;
mod arguments;
mod constraints;
mod context;
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
    generated_tokens: GeneratorResult,
}

impl TokenGenerator {
    pub fn new(context: Context) -> Result<Self, syn::Error> {
        let mut generators = [
            ConstraintGenerators::Args(ArgumentsGenerator::new()),
            ConstraintGenerators::Rent(RentGenerator::new()),
            ConstraintGenerators::Assign(AssignGenerator::new()),
            ConstraintGenerators::Bumps(Box::new(BumpsGenerator::new())),
            ConstraintGenerators::Init(InitializationGenerator::new()),
            ConstraintGenerators::HasOne(HasOneGenerator::new()),
        ];

        let mut generated_tokens = GeneratorResult::default();
        for generator in &mut generators {
            generator.visit_context(&context)?;
            let generated = generator.generate()?;

            if !generated.new_fields.is_empty() {
                generated_tokens
                    .new_fields
                    .reserve(generated.new_fields.len());
                generated_tokens.new_fields.extend(generated.new_fields);
            }

            if !generated.at_init.is_empty() {
                generated_tokens.at_init.extend(generated.at_init);
            }
            if !generated.after_init.is_empty() {
                generated_tokens.after_init.extend(generated.after_init);
            }
            if !generated.global_outside.is_empty() {
                generated_tokens
                    .global_outside
                    .extend(generated.global_outside);
            }
        }

        Ok(TokenGenerator {
            context,
            generated_tokens,
        })
    }
}

impl ToTokens for TokenGenerator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.context.item_struct.ident;
        let generics = &self.context.item_struct.generics;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let new_lifetime: Lifetime = parse_quote!('info);

        let global_outside = &self.generated_tokens.global_outside;
        let at_init = &self.generated_tokens.at_init;
        let after_init = &self.generated_tokens.after_init;

        let name_list: Vec<&Ident> = self
            .context
            .item_struct
            .fields
            .iter()
            .filter_map(|f| f.ident.as_ref())
            .collect();

        let mut struct_fields: Vec<&Ident> = name_list.clone();

        let account_struct = &mut self.context.item_struct.to_owned();
        for new_field in &self.generated_tokens.new_fields {
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

                    #at_init

                    #after_init

                    *accounts = rem;

                    Ok(#name { #(#struct_fields),* })
                }
            }
        };

        let doc = prettyplease::unparse(
            &syn::parse2::<syn::File>(quote! {
                #global_outside

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
            #global_outside

            #account_struct

            #impl_context

        };
        expanded.to_tokens(tokens);
    }
}

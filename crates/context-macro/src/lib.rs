use {
    crate::cross_checks::cross_checks,
    context::Context,
    generators::*,
    injector::FieldInjector,
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    sorter::sort_accounts,
    syn::{parse_macro_input, parse_quote, visit_mut::VisitMut, Attribute, Ident},
};

mod accounts;
mod context;
mod cross_checks;
mod extractor;
mod generators;
mod injector;
mod remover;
mod sorter;
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
    fn append(&mut self, result: &mut GeneratorResult) -> Result<(), syn::Error>;
}

impl TokenGenerator {
    pub fn new(mut context: Context) -> Result<Self, syn::Error> {
        sort_accounts(&mut context)?;

        let mut generated_results = GeneratorResult::default();
        let mut generators = [
            ConstraintGenerators::Args(ArgumentsGenerator::new(&context)),
            ConstraintGenerators::Assign(AssignGenerator::new(&context)),
            ConstraintGenerators::Rent(RentGenerator::new(&context)),
            ConstraintGenerators::Init(InitGenerator::new(&context)),
            ConstraintGenerators::Bumps(BumpsGenerator::new(&context)),
            ConstraintGenerators::HasOne(HasOneGenerator::new(&context)),
            ConstraintGenerators::Token(TokenAccountGenerator::new(&context)),
        ];

        cross_checks(&context)?;

        for generator in &mut generators {
            generator.append(&mut generated_results)?;
        }

        Ok(TokenGenerator {
            context,
            result: generated_results,
        })
    }
}

impl ToTokens for TokenGenerator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.context.item_struct.ident;
        let generics = &self.context.item_struct.generics;

        let (_, ty_generics, _) = generics.split_for_impl();

        // patch the lifetime of the new context here
        let generics = &mut generics.to_owned();
        generics.params.push(parse_quote!('c));
        if let Some(where_clause) = &mut generics.where_clause {
            where_clause.predicates.push(parse_quote!('c: 'info));
        } else {
            generics.where_clause = Some(parse_quote!(where 'c: 'info));
        }
        let (impl_generics, _, where_clause) = generics.split_for_impl();

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
        let drop_vars = self.result.drop_vars.iter().map(|v| quote!(drop(#v);));

        let impl_context = quote! {
            impl #impl_generics HandlerContext<'_, 'info, 'c> for #name #ty_generics #where_clause {
                #[inline(always)]
                fn from_entrypoint(
                    program_id: &Pubkey,
                    accounts: &mut &'info [AccountInfo],
                    instruction_data: &mut &'c [u8],
                ) -> ProgramResult<Self> {
                    let [#(#name_list,)* rem @ ..] = accounts else {
                        return Err(ProgramError::NotEnoughAccountKeys.into());
                    };

                    #inside

                    #(#drop_vars)*

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

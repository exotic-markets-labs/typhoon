use {
    crate::cross_checks::cross_checks,
    context::Context,
    generators::*,
    injector::FieldInjector,
    proc_macro::TokenStream,
    proc_macro2::Span,
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

        let mut destructuring_vars = Vec::new();
        let mut fixed_account_count = 0usize;
        let mut has_const_generic_arrays = false;
        let mut const_generic_arrays = Vec::new();

        for account in &self.context.accounts {
            if account.is_array {
                if let Some(const_generic) = &account.const_generic {
                    has_const_generic_arrays = true;
                    const_generic_arrays.push((account.name.clone(), const_generic.clone()));
                } else if let Some(size) = account.array_size {
                    for i in 0..size {
                        let var_name = format!("{}_{}", account.name, i);
                        let var_ident = Ident::new(&var_name, account.name.span());
                        destructuring_vars.push(var_ident);
                        fixed_account_count += 1;
                    }
                }
            } else {
                destructuring_vars.push(account.name.clone());
                fixed_account_count += 1;
            }
        }

        fn generate_const_generic_splitting(
            const_generic_arrays: &[(Ident, Ident)],
            fixed_account_count: usize,
            context: &Context,
        ) -> proc_macro2::TokenStream {
            let mut splitting_code = Vec::new();

            if fixed_account_count > 0 {
                let destructuring_vars = (0..fixed_account_count)
                    .map(|i| Ident::new(&format!("fixed_{}", i), Span::call_site()))
                    .collect::<Vec<_>>();

                splitting_code.push(quote! {
                    let [#(#destructuring_vars,)* remaining_accounts @ ..] = accounts else {
                        return Err(ProgramError::NotEnoughAccountKeys.into());
                    };
                });
            } else {
                splitting_code.push(quote! {
                    let remaining_accounts = accounts;
                });
            }

            // For const generic arrays, create the actual arrays
            for (array_name, const_param) in const_generic_arrays {
                // Find the account type and optional flag for this array
                let (account_ty, is_optional) = context
                    .accounts
                    .iter()
                    .find(|acc| acc.name == *array_name)
                    .map(|acc| (&acc.ty, acc.is_optional))
                    .unwrap_or_else(|| panic!("Array account not found: {}", array_name));

                let array_creation = if is_optional {
                    quote! {
                        let #array_name = core::array::from_fn::<_, #const_param, _>(|i| {
                            let account_info = &remaining_accounts[i];
                            if account_info.key() == program_id {
                                None
                            } else {
                                Some(<#account_ty as FromAccountInfo>::try_from_info(account_info).trace_account(stringify!(#array_name)).unwrap())
                            }
                        });
                    }
                } else {
                    quote! {
                        let #array_name = core::array::from_fn::<_, #const_param, _>(|i| {
                            <#account_ty as FromAccountInfo>::try_from_info(&remaining_accounts[i]).trace_account(stringify!(#array_name)).unwrap()
                        });
                    }
                };

                splitting_code.push(quote! {
                    if remaining_accounts.len() < #const_param {
                        return Err(ProgramError::NotEnoughAccountKeys.into());
                    }
                    #array_creation
                    remaining_accounts = &remaining_accounts[#const_param..];
                });
            }

            quote! {
                #(#splitting_code)*
            }
        }

        let impl_context = if has_const_generic_arrays {
            let account_splitting_code = generate_const_generic_splitting(
                &const_generic_arrays,
                fixed_account_count,
                &self.context,
            );

            quote! {
                impl #impl_generics HandlerContext<'_, 'info, 'c> for #name #ty_generics #where_clause {
                    #[inline(always)]
                    fn from_entrypoint(
                        program_id: &Pubkey,
                        accounts: &mut &'info [AccountInfo],
                        instruction_data: &mut &'c [u8],
                    ) -> ProgramResult<Self> {
                        let min_accounts = #fixed_account_count;
                        if accounts.len() < min_accounts {
                            return Err(ProgramError::NotEnoughAccountKeys.into());
                        }

                        #account_splitting_code

                        #inside

                        #(#drop_vars)*

                        Ok(#name { #(#struct_fields),* })
                    }
                }
            }
        } else {
            quote! {
                impl #impl_generics HandlerContext<'_, 'info, 'c> for #name #ty_generics #where_clause {
                    #[inline(always)]
                    fn from_entrypoint(
                        program_id: &Pubkey,
                        accounts: &mut &'info [AccountInfo],
                        instruction_data: &mut &'c [u8],
                    ) -> ProgramResult<Self> {
                        let expected_accounts = #fixed_account_count;
                        if accounts.len() < expected_accounts {
                            return Err(ProgramError::NotEnoughAccountKeys.into());
                        }

                        let [#(#destructuring_vars,)* rem @ ..] = accounts else {
                            return Err(ProgramError::NotEnoughAccountKeys.into());
                        };

                        #inside

                        #(#drop_vars)*

                        *accounts = rem;

                        Ok(#name { #(#struct_fields),* })
                    }
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

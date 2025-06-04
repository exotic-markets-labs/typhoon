use {
    crate::{generator::Generator, instruction::Instruction},
    heck::ToUpperCamelCase,
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    std::collections::HashMap,
    syn::Ident,
    typhoon_syn::arguments::{Argument, Arguments},
};

pub struct ClientGenerator(HashMap<String, TokenStream>);

impl ClientGenerator {
    fn generate_args(
        &mut self,
        args: &[(Ident, Arguments)],
    ) -> (Vec<TokenStream>, Vec<TokenStream>) {
        args.iter()
            .enumerate()
            .map(|(i, (context_name, ty))| {
                let ty_name= match ty {
                    Arguments::Struct(ident) => ident.clone(),
                    Arguments::Values(args) => {
                        let struct_name = format_ident!("{context_name}Args");
                        let name_str = struct_name.to_string();
                        self.0.entry(name_str).or_insert_with(|| {
                            let fields = args
                            .iter()
                            .map(|Argument { name, ty }: &Argument| quote!(pub #name: #ty));
                            let item = quote! {
                                #[derive(Debug, PartialEq, bytemuck::AnyBitPattern, bytemuck::NoUninit, Copy, Clone)]
                                #[repr(C)]
                                pub struct #struct_name {
                                    #(#fields),*
                                }
                            };
                            item

                        });

                        format_ident!("{context_name}Args")
                    }
                };
                let var_name = format_ident!("arg_{i}");
                (
                    quote!(pub #var_name: #ty_name,),
                    quote!(data.extend_from_slice(bytemuck::bytes_of(&self.#var_name));),
                )
            })
            .collect()
    }

    fn generate_accounts(
        accounts: &[(Ident, (bool, bool, bool))],
    ) -> (Vec<TokenStream>, Vec<TokenStream>) {
        accounts.iter().map(|(name, (is_optional, is_mutable,is_signer))| {
            let field = if *is_optional {
                quote!(pub #name: Option<Pubkey>,)
            }else {
                quote!(pub #name: Pubkey,)
            };

            let push = if *is_optional {
                let meta = if *is_mutable {
                    quote!(accounts.push(solana_instruction::AccountMeta::new(#name, #is_signer));)
                }else {
                    quote!(accounts.push(solana_instruction::AccountMeta::new_readonly(#name, #is_signer));)
                };
                quote! {
                    if let Some(#name) = self.#name {
                        #meta
                    }else {
                        accounts.push(solana_instruction::AccountMeta::new_readonly(solana_pubkey::Pubkey::default(), false));
                    }
                }
            }else {
                if *is_mutable {
                    quote!(accounts.push(solana_instruction::AccountMeta::new(self.#name, #is_signer));)
                }else {
                    quote!(accounts.push(solana_instruction::AccountMeta::new_readonly(self.#name, #is_signer));)
                }
            };

            (field, push)
        }).collect()
    }
}

impl Generator for ClientGenerator {
    fn generate_token(ix: &[(usize, Instruction)]) -> TokenStream {
        let mut token = TokenStream::new();
        let mut generator = ClientGenerator(HashMap::new());
        for (discriminator, instruction) in ix {
            let (arg_fields, arg_extend) = generator.generate_args(&instruction.args);
            let account_len = instruction.accounts.len();
            let (account_fields, account_push) =
                ClientGenerator::generate_accounts(&instruction.accounts);
            let name = format_ident!(
                "{}Instruction",
                instruction.name.to_string().to_upper_camel_case()
            );
            let dis = *discriminator as u8;
            token.extend(quote! {
                pub struct #name {
                    #(#arg_fields)*
                    #(#account_fields)*
                }

                impl #name {
                    pub fn into_instruction(self) -> solana_instruction::Instruction {
                        let mut data = vec![#dis];
                        #(#arg_extend)*

                        let mut accounts = Vec::with_capacity(#account_len);
                        #(#account_push)*

                        solana_instruction::Instruction {
                            program_id: crate::ID.into(),
                            accounts,
                            data
                        }
                    }
                }
            });
        }
        token.extend(generator.0.into_values());
        token
    }
}

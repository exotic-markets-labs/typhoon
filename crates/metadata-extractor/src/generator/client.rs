use {
    crate::{generator::Generator, instruction::Instruction},
    heck::ToUpperCamelCase,
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::Ident,
};

pub struct ClientGenerator;

impl ClientGenerator {
    fn generate_args(args: &[Ident]) -> (Vec<TokenStream>, Vec<TokenStream>) {
        args.iter()
            .enumerate()
            .map(|(i, ty)| {
                let var_name = format_ident!("arg_{i}");
                (
                    quote!(pub var_name: #ty,),
                    quote!(data.extend_from_slice(self.#var_name);),
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

            let meta = if *is_mutable {
                quote!(solana_instruction::AccountMeta::new(self.#name, #is_signer))
            }else {
                quote!(solana_instruction::AccountMeta::new_readonly(self.#name, #is_signer))
            };
            let push = if *is_optional {
                quote! {
                    if let Some(#name) = self.#name {
                        accounts.push(#meta);
                    }else {
                        accounts.push(solana_instruction::AccountMeta::new_readonly(solana_pubkey::Pubkey::default(), false));
                    }
                }
            }else {
                quote!(accounts.push(#meta);)
            };

            (field, push)
        }).collect()
    }
}

impl Generator for ClientGenerator {
    fn generate_token(ix: &[(usize, Instruction)]) -> TokenStream {
        let mut token = TokenStream::new();
        for (discriminator, instruction) in ix {
            let (arg_fields, arg_extend) = ClientGenerator::generate_args(&instruction.args);
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
        token
    }
}

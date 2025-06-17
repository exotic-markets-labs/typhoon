use {
    crate::{
        generator::{generate_argument, Generator},
        instruction::{Instruction, InstructionArg},
    },
    heck::ToUpperCamelCase,
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    std::collections::HashMap,
    syn::Ident,
};

pub struct CpiGenerator(HashMap<String, TokenStream>);

impl CpiGenerator {
    fn generate_arguments(
        &mut self,
        args: &[InstructionArg],
    ) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
        let mut lens = Vec::with_capacity(args.len());
        let mut fields = Vec::with_capacity(args.len());
        let mut bytes = Vec::with_capacity(args.len());

        for (i, arg) in args.iter().enumerate() {
            let (ty, item_res) = generate_argument(arg);
            if let Some((name_str, item)) = item_res {
                self.0.entry(name_str).or_insert_with(|| item);
            }
            let var_name = format_ident!("arg_{i}");
            lens.push(quote!(core::mem::size_of::<#ty>()));
            fields.push(quote!(pub #var_name: &'a #ty,));
            bytes.push(quote!(bytemuck::bytes_of(self.#var_name)));
        }
        (lens, fields, bytes)
    }

    fn generate_accounts(
        accounts: &[(Ident, (bool, bool, bool))],
    ) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
        let mut fields = Vec::with_capacity(accounts.len());
        let mut metas = Vec::with_capacity(accounts.len());
        let mut infos = Vec::with_capacity(accounts.len());
        for (name, (is_optional, is_mutable, is_signer)) in accounts {
            let field = if *is_optional {
                quote!(pub #name: Option<&'a AccountInfo>,)
            } else {
                quote!(pub #name: &'a AccountInfo,)
            };
            let meta = if *is_optional {
                quote! {
                    if let Some(#name) = self.#name {
                        instruction::AccountMeta::new(#name.key(), #is_mutable, #is_signer)
                    }else {
                        instruction::AccountMeta::new(self.program.key(), false, false)
                    }
                }
            } else {
                quote!(instruction::AccountMeta::new(self.#name.key(), #is_mutable, #is_signer))
            };

            let info = if *is_optional {
                quote!(self.#name.unwrap_or(self.program))
            } else {
                quote!(self.#name)
            };

            fields.push(field);
            metas.push(meta);
            infos.push(info);
        }
        (fields, metas, infos)
    }
}

impl Generator for CpiGenerator {
    fn generate_token(ix: &[(usize, Instruction)]) -> TokenStream {
        let mut token = TokenStream::new();
        let mut generator = CpiGenerator(HashMap::new());

        for (discriminator, instruction) in ix {
            let instruction_name =
                format_ident!("{}Cpi", instruction.name.to_string().to_upper_camel_case());
            let dis = *discriminator as u8;
            let (lens, argument_fields, bytes) = generator.generate_arguments(&instruction.args);
            let (account_fields, metas, infos) =
                CpiGenerator::generate_accounts(&instruction.accounts);
            let len = if lens.is_empty() {
                quote!(1)
            } else {
                quote!(#(#lens)+*)
            };

            token.extend(quote! {
                pub struct #instruction_name<'a> {
                    #(#argument_fields)*
                    #(#account_fields)*
                    pub program: &'a AccountInfo,
                }

                impl #instruction_name<'_> {
                    #[inline(always)]
                    pub fn invoke(&self) -> ProgramResult {
                        self.invoke_signed(&[])
                    }

                    #[inline(always)]
                    pub fn invoke_signed(&self, seeds: &[instruction::CpiSigner]) -> ProgramResult {
                        let mut bytes = [bytes::UNINIT_BYTE; #len];
                        let mut writer = bytes::MaybeUninitWriter::new(&mut bytes, 0);
                        writer.write_bytes(&[#dis])?;
                        #(writer.write_bytes(#bytes)?;)*

                        let instruction = instruction::Instruction {
                            program_id: &self.program.key(),
                            data: writer.initialized(),
                            accounts: &[
                                #(#metas),*
                            ]
                        };

                        invoke_signed(
                            &instruction,
                            &[
                                #(#infos),*
                            ],
                            seeds
                        ).map_err(Into::into)
                    }
                }
            });
        }
        token.extend(generator.0.into_values());
        token
    }
}

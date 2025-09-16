use {
    crate::anchor::{gen_docs, gen_type_ref},
    anchor_lang_idl_spec::{Idl, IdlField, IdlInstructionAccountItem},
    five8_const::decode_32_const,
    heck::ToUpperCamelCase,
    proc_macro2::{Span, TokenStream},
    quote::quote,
    syn::{Expr, Ident},
};

pub fn gen_instructions(idl: &Idl) -> TokenStream {
    let program_id = &idl.address;
    let instructions = idl.instructions.iter().map(|instruction| {
        let name = instruction.name.to_upper_camel_case();
        let ident = Ident::new(&name, Span::call_site());
        let (metas, accounts) = gen_account_instruction(&instruction.accounts);
        let docs = gen_docs(&instruction.docs);

        let account_metas = gen_account_metas(&metas);
        let discriminator = &instruction.discriminator;
        let (arg_fields, instruction_data) =
            gen_instruction_data(&instruction.args, discriminator, program_id);

        quote! {
            /// Used for Cross-Program Invocation (CPI) calls.
            #docs
            pub struct #ident<'a> {
                #(pub #accounts: &'a AccountInfo,)*
                #(#arg_fields)*
            }

            impl #ident<'_> {
                #[inline(always)]
                pub fn invoke(&self) -> ProgramResult {
                    self.invoke_signed(&[])
                }

                pub fn invoke_signed(&self, seeds: &[instruction::CpiSigner]) -> ProgramResult {
                    #account_metas
                    #instruction_data

                    invoke_signed(
                        &instruction,
                        &[#(self.#accounts),*],
                        seeds
                    ).map_err(Into::into)
                }
            }
        }
    });

    quote! {
        #(#instructions)*
    }
}

fn gen_instruction_data(
    args: &[IdlField],
    discriminator: &[u8],
    program_id: &str,
) -> (Vec<TokenStream>, TokenStream) {
    let id_array = decode_32_const(program_id);
    let discriminator_len = discriminator.len();
    let buffer_size = 1232 - discriminator_len;
    let discriminator_expr: Expr = syn::parse_quote!(&[#(#discriminator),*]);
    let (arg_fields, arg_ser): (Vec<TokenStream>, Vec<TokenStream>) = args
        .iter()
        .map(|arg| {
            let ident = Ident::new(&arg.name, Span::call_site());
            let ty_ref = gen_type_ref(&arg.ty);

            (
                quote!(pub #ident: #ty_ref,),
                quote!(borsh::ser::BorshSerialize::serialize(&self.#ident, &mut writer).map_err(|_| ProgramError::BorshIoError)?;),
            )
        })
        .unzip();

    let instruction_data = if arg_ser.is_empty() {
        quote! {
            let mut instruction_data = [bytes::UNINIT_BYTE; #discriminator_len];

            bytes::write_bytes(&mut instruction_data, #discriminator_expr);

            let instruction = instruction::Instruction {
                program_id: &[#(#id_array),*],
                accounts: &account_metas,
                data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, #discriminator_len) },
            };
        }
    } else {
        quote! {
            let mut instruction_data = [bytes::UNINIT_BYTE; #buffer_size];
            bytes::write_bytes(&mut instruction_data, #discriminator_expr);

            let mut writer = bytes::MaybeUninitWriter::new(&mut instruction_data, #discriminator_len);
            #(#arg_ser)*

            let instruction = instruction::Instruction {
                program_id: &[#(#id_array),*],
                accounts: &account_metas,
                data: writer.initialized(),
            };
        }
    };

    (arg_fields, instruction_data)
}

fn gen_account_instruction(
    accounts: &[IdlInstructionAccountItem],
) -> (Vec<TokenStream>, Vec<syn::Ident>) {
    let mut metas = Vec::with_capacity(accounts.len());
    let mut fields = Vec::with_capacity(accounts.len());

    for account in accounts {
        match account {
            IdlInstructionAccountItem::Composite(composite_accounts) => {
                let (nested_metas, nested_fields) =
                    gen_account_instruction(&composite_accounts.accounts);
                metas.extend(nested_metas);
                fields.extend(nested_fields);
            }
            IdlInstructionAccountItem::Single(account) => {
                let ident = Ident::new(&account.name, Span::call_site());
                let is_writable = account.writable;
                let is_signer = account.signer;

                metas.push(quote! {
                    instruction::AccountMeta::new(self.#ident.key(), #is_writable, #is_signer)
                });
                fields.push(ident);
            }
        }
    }

    (metas, fields)
}

#[inline]
fn gen_account_metas(metas: &[TokenStream]) -> TokenStream {
    let len = metas.len();

    quote! {
        let account_metas: [instruction::AccountMeta; #len] = [#(#metas),*];
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        anchor_lang_idl_spec::{IdlInstructionAccount, IdlType},
    };

    #[test]
    fn test_gen_instruction_data() {
        let args = vec![];
        let discriminator = vec![1, 2, 3, 4];
        let program_id = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS";

        let (fields, data) = gen_instruction_data(&args, &discriminator, program_id);
        let expected_data = quote! {
            let mut instruction_data = [bytes::UNINIT_BYTE; 4usize];

            bytes::write_bytes(&mut instruction_data, &[1u8, 2u8, 3u8, 4u8]);

            let instruction = instruction::Instruction {
                program_id: &[218u8, 7u8, 92u8, 178u8, 255u8, 94u8, 198u8, 129u8, 118u8, 19u8, 222u8, 83u8, 11u8, 105u8, 42u8, 135u8, 53u8, 71u8, 119u8, 105u8, 218u8, 71u8, 67u8, 12u8, 189u8, 129u8, 84u8, 51u8, 92u8, 74u8, 131u8, 39u8],
                accounts: &account_metas,
                data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 4usize) },
            };
        };
        assert!(fields.is_empty());
        assert_eq!(data.to_string(), expected_data.to_string());

        let args = vec![IdlField {
            docs: vec![],
            name: "amount".to_string(),
            ty: IdlType::U64,
        }];
        let discriminator = vec![1, 2, 3, 4];
        let program_id = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS";

        let (fields, data) = gen_instruction_data(&args, &discriminator, program_id);
        let expected_data = quote! {
            let mut instruction_data = [bytes::UNINIT_BYTE; 1228usize];
            bytes::write_bytes(&mut instruction_data, &[1u8, 2u8, 3u8, 4u8]);

            let mut writer = bytes::MaybeUninitWriter::new(&mut instruction_data, 4usize);
            borsh::ser::BorshSerialize::serialize(&self.amount, &mut writer).map_err(|_| ProgramError::BorshIoError)?;

            let instruction = instruction::Instruction {
                program_id: &[218u8, 7u8, 92u8, 178u8, 255u8, 94u8, 198u8, 129u8, 118u8, 19u8, 222u8, 83u8, 11u8, 105u8, 42u8, 135u8, 53u8, 71u8, 119u8, 105u8, 218u8, 71u8, 67u8, 12u8, 189u8, 129u8, 84u8, 51u8, 92u8, 74u8, 131u8, 39u8],
                accounts: &account_metas,
                data: writer.initialized(),
            };
        };

        assert_eq!(fields.len(), 1);
        assert_eq!(data.to_string(), expected_data.to_string());
    }

    #[test]
    fn test_gen_account_instruction() {
        let accounts = vec![
            IdlInstructionAccountItem::Single(IdlInstructionAccount {
                name: "test_account".to_string(),
                writable: true,
                signer: false,
                docs: vec![],
                optional: false,
                address: None,
                pda: None,
                relations: vec![],
            }),
            IdlInstructionAccountItem::Single(IdlInstructionAccount {
                name: "test_account2".to_string(),
                writable: false,
                signer: true,
                docs: vec![],
                optional: false,
                address: None,
                pda: None,
                relations: vec![],
            }),
        ];

        let (metas, fields) = gen_account_instruction(&accounts);

        let result = quote! {
            #(#metas),*
        };
        let expected = quote! {
            instruction::AccountMeta::new(self.test_account.key(), true, false),
            instruction::AccountMeta::new(self.test_account2.key(), false, true)
        };

        assert_eq!(result.to_string(), expected.to_string());
        assert_eq!(fields[0].to_string(), "test_account");
        assert_eq!(fields[1].to_string(), "test_account2");
    }

    #[test]
    fn test_gen_account_metas() {
        let metas = vec![quote!(meta1), quote!(meta2)];
        let result = gen_account_metas(&metas);
        let expected = quote! {
            let account_metas: [instruction::AccountMeta; 2usize] = [meta1, meta2];
        };

        assert_eq!(result.to_string(), expected.to_string());
    }
}

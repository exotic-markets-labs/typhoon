use {
    crate::anchor::{gen_docs, gen_type, gen_type_ref},
    anchor_lang_idl_spec::{Idl, IdlField, IdlInstructionAccountItem, IdlType},
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
        let program_result = gen_instruction_result(&instruction.returns);

        let account_metas = gen_account_metas(&metas);
        let discriminator = &instruction.discriminator;
        let (arg_fields, instruction_data) =
            gen_instruction_data(&instruction.args, discriminator, program_id);

        quote! {
            /// Used for Cross-Program Invocation (CPI) calls.
            #docs
            pub struct #ident<'a> {
                #(pub #accounts: &'a program::RawAccountInfo,)*
                #(#arg_fields)*
            }

            impl #ident<'_> {
                #[inline(always)]
                pub fn invoke(&self) -> #program_result {
                    self.invoke_signed(&[])
                }

                pub fn invoke_signed(&self, seeds: &[program::SignerSeeds]) -> #program_result {
                    #account_metas
                    #instruction_data

                    program::invoke_signed(
                        &instruction,
                        &[#(self.#accounts),*],
                        seeds
                    )
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
                quote!(borsh::ser::BorshSerialize::serialize(self.#ident, &mut writer).map_err(|_| Error::BorshIoError)?;),
            )
        })
        .unzip();

    let instruction_data = if arg_ser.is_empty() {
        quote! {
            let mut instruction_data = [program::bytes::UNINIT_BYTE; #discriminator_len];

            program::bytes::write_bytes(&mut instruction_data, #discriminator_expr);

            let instruction = program::Instruction {
                program_id: &program::pubkey_from_array([#(#id_array),*]),
                accounts: &account_metas,
                data: unsafe { std::slice::from_raw_parts(instruction_data.as_ptr() as _, #discriminator_len) },
            };
        }
    } else {
        quote! {
            let mut instruction_data = [program::bytes::UNINIT_BYTE; #buffer_size];
            program::bytes::write_bytes(&mut instruction_data, #discriminator_expr);

            let mut writer = program::bytes::MaybeUninitWriter::new(&mut instruction_data, #discriminator_len);
            #(#arg_ser)*

            let instruction = program::Instruction {
                program_id: &program::pubkey_from_array([#(#id_array),*]),
                accounts: &account_metas,
                data: writer.initialized(),
            };
        }
    };

    (arg_fields, instruction_data)
}

fn gen_instruction_result(returns: &Option<IdlType>) -> TokenStream {
    match returns {
        Some(ty) => {
            let result_ty = gen_type(ty);
            quote!(Result<#result_ty, program::program_error::ProgramError>)
        }
        None => quote!(program::ProgramResult),
    }
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
                    program::ToMeta::to_meta(self.#ident, #is_writable, #is_signer)
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
        let account_metas: [program::AccountMeta; #len] = [#(#metas),*];
    }
}

#[cfg(test)]
mod tests {
    use {super::*, anchor_lang_idl_spec::IdlInstructionAccount};

    #[test]
    fn test_gen_instruction_data() {
        let args = vec![];
        let discriminator = vec![1, 2, 3, 4];
        let program_id = "test_program";

        let (fields, data) = gen_instruction_data(&args, &discriminator, program_id);
        let expected_data = quote! {
            let mut instruction_data = [program::bytes::UNINIT_BYTE; 4usize];

            program::bytes::write_bytes(&mut instruction_data, &[1u8, 2u8, 3u8, 4u8]);

            let instruction = program::Instruction {
                program_id: &program::pubkey!(#program_id),
                accounts: &account_metas,
                data: unsafe { std::slice::from_raw_parts(instruction_data.as_ptr() as _, 4usize) },
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
        let program_id = "test_program";

        let (fields, data) = gen_instruction_data(&args, &discriminator, program_id);
        let expected_data = quote! {
            let mut instruction_data = [program::bytes::UNINIT_BYTE; 1228usize];
            program::bytes::write_bytes(&mut instruction_data, &[1u8, 2u8, 3u8, 4u8]);

            let mut writer = program::bytes::MaybeUninitWriter::new(&mut instruction_data, 4usize);
            borsh::ser::BorshSerialize::serialize(self.amount, &mut writer).map_err(|_| Error::BorshIoError)?;

            let instruction = program::Instruction {
                program_id: &program::pubkey!(#program_id),
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
            program::ToMeta::to_meta(self.test_account, true, false),
            program::ToMeta::to_meta(self.test_account2, false, true)
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
            let account_metas: [program::AccountMeta; 2usize] = [meta1, meta2];
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_gen_instruction_result() {
        let result_none = gen_instruction_result(&None);
        let expected_none = quote!(program::ProgramResult);
        assert_eq!(result_none.to_string(), expected_none.to_string());

        let result_some = gen_instruction_result(&Some(IdlType::Bool));
        let expected_some = quote!(Result<bool, program::program_error::ProgramError>);
        assert_eq!(result_some.to_string(), expected_some.to_string());
    }
}

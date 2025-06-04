// use {
//     crate::{generator::Generator, instruction::Instruction},
//     heck::ToUpperCamelCase,
//     proc_macro2::TokenStream,
//     quote::{format_ident, quote},
//     syn::Ident,
// };

// // TODO optional accounts
// pub struct CpiGenerator;

// impl CpiGenerator {
//     fn generate_accounts(
//         accounts: &[(Ident, (bool, bool, bool))],
//     ) -> (TokenStream, TokenStream, TokenStream) {
//         let mut fields = TokenStream::new();
//         let mut metas = TokenStream::new();
//         let mut elements = TokenStream::new();

//         for (name, (_, is_mutable, is_signer)) in accounts {
//             fields.extend(quote!(pub #name: &'a RawAccountInfo,));
//             metas.extend(quote!(AccountMeta::new(self.#name.key(), #is_mutable, #is_signer),));
//             elements.extend(quote!(self.#name,));
//         }
//         (fields, metas, elements)
//     }
// }

// impl Generator for CpiGenerator {
//     fn generate_token(ix: &[(usize, Instruction)]) -> TokenStream {
//         let mut token = TokenStream::new();
//         for (discriminator, instruction) in ix {
//             let instruction_name =
//                 format_ident!("{}Cpi", instruction.name.to_string().to_upper_camel_case());
//             let (args_name) = instruction.args.iter().enumerate().map(|(i, ty)| {
//                 let var_name = format_ident!("arg_{i}");
//                 var_name
//             });
//             let (account_fields, _, elements) =
//                 CpiGenerator::generate_accounts(&instruction.accounts);
//             token.extend(quote! {
//                 pub struct #instruction_name<'a> {
//                     #account_fields
//                 }

//                 impl #instruction_name<'_> {
//                     pub fn invoke(&self) -> ProgramResult {
//                         self.invoke_signed(&[])
//                     }

//                     pub fn invoke_signed(&self, seeds: &[SignerSeeds]) -> ProgramResult {
//                         let mut instruction_data = [bytes::UNINIT_BYTE; 34];

//                         let instruction = Instruction {
//                             program_id: &crate::ID,
//                             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 0) },
//                             accounts: &[

//                             ]
//                         }

//                         invoke_signed(
//                             &instruction,
//                             &[
//                                 #elements
//                             ],
//                             seeds,
//                         )
//                     }
//                 }
//             });
//         }

//         token
//     }
// }

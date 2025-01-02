use {
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Path, Token},
};

#[proc_macro]
pub fn handlers(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as Handlers)
        .to_token_stream()
        .into()
}

struct Handlers {
    instructions: Punctuated<Path, Token![,]>,
}

impl Parse for Handlers {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let instructions = Punctuated::<Path, Token![,]>::parse_terminated(input)?;

        Ok(Handlers { instructions })
    }
}

impl ToTokens for Handlers {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let instructions = self.instructions.iter().enumerate().map(|(i, val)| {
            let i = i as u64;
            quote! {
                #i => handle(accounts, instruction_data_inner, #val)?,
            }
        });

        let expanded = quote! {
            typhoon_program::program_entrypoint!(process_instruction);

            pub fn process_instruction(
                _program_id: &typhoon_program::pubkey::Pubkey,
                accounts: &[typhoon_program::RawAccountInfo],
                instruction_data: &[u8],
            ) -> typhoon_program::ProgramResult {
                let (discriminator, instruction_data_inner) =
                u64::ref_from_prefix(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;
                match discriminator {
                    #(#instructions)*
                    _ => {
                        typhoon_program::msg!("Error: unknown instruction") //TODO
                    },
                }
                Ok(())
            }
        };

        expanded.to_tokens(tokens);
    }
}

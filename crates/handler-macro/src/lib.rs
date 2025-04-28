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
            let i = i as u8;
            quote! {
                #i => handle(accounts, data, #val)?,
            }
        });

        let expanded = quote! {
            entrypoint!(process_instruction);

            pub fn process_instruction(
                program_id: &Pubkey,
                accounts: &[AccountInfo],
                instruction_data: &[u8],
            ) -> Result<(), ProgramError> {
                if program_id != &crate::ID {
                    return Err(ProgramError::IncorrectProgramId);
                }

                let (discriminator, data) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
                match discriminator {
                    #(#instructions)*
                    _ => return Err(ProgramError::InvalidInstructionData),
                }

                Ok(())
            }
        };

        expanded.to_tokens(tokens);
    }
}

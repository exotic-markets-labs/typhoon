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
                #i => crayfish_context::handle(accounts, instruction_data_inner, #val)?,
            }
        });

        let expanded = quote! {
            crayfish_program::program_entrypoint!(process_instruction);

            pub fn process_instruction(
                _program_id: &crayfish_program::pubkey::Pubkey,
                accounts: &[crayfish_program::RawAccountInfo],
                instruction_data: &[u8],
            ) -> crayfish_program::ProgramResult {
                let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);
                match instruction_discriminant[0] {
                    #(#instructions)*
                    _ => {
                        crayfish_program::msg!("Error: unknown instruction") //TODO
                    },
                }
                Ok(())
            }
        };

        expanded.to_tokens(tokens);
    }
}

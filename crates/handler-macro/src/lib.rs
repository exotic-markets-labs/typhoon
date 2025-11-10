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
    inline: bool,
}

impl Parse for Handlers {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse paths until we encounter "no_inline" or the input is empty
        // TODO: Remove this once we have a better borsh lib
        let mut instructions = Punctuated::<Path, Token![,]>::new();
        let mut inline = true;

        if input.is_empty() {
            return Ok(Self {
                instructions,
                inline,
            });
        }

        instructions.push_value(input.parse()?);

        while input.peek(Token![,]) {
            let comma: Token![,] = input.parse()?;

            if let Ok(lit_str) = input.parse::<syn::LitStr>() {
                if lit_str.value() == "no_inline" {
                    inline = false;

                    if input.peek(Token![,]) {
                        let _ = input.parse::<Token![,]>();
                    }

                    break;
                } else {
                    return Err(syn::Error::new_spanned(
                        lit_str,
                        "Expected either a path to a handler or'no_inline' to disable inline attribute",
                    ));
                }
            }

            instructions.push_punct(comma);
            instructions.push_value(input.parse()?);
        }

        Ok(Handlers {
            instructions,
            inline,
        })
    }
}

impl ToTokens for Handlers {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let instructions = self.instructions.iter().enumerate().map(|(i, val)| {
            let i = i as u8;
            quote! {
                #i => handle(program_id, accounts, data, #val),
            }
        });

        let inline = if self.inline {
            quote! {
                #[inline(always)]
            }
        } else {
            quote! {
                #[inline(never)]
            }
        };

        let expanded = quote! {
            program_entrypoint!(process_instruction);

            #inline
            pub fn process_instruction(
                program_id: &Pubkey,
                accounts: &[AccountInfo],
                instruction_data: &[u8],
            ) -> Result<(), ProgramError> {
                let (discriminator, data) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
                let result = match discriminator {
                    #(#instructions)*
                    _ => Err(ErrorCode::UnknownInstruction.into()),
                };

                #[cfg(feature = "logging")]
                result.inspect_err(log_error)?;

                #[cfg(not(feature = "logging"))]
                result?;

                Ok(())
            }
        };

        expanded.to_tokens(tokens);
    }
}

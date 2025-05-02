use {
    crate::{
        arguments::{Argument, Arguments},
        GenerationContext, StagedGenerator,
    },
    quote::{format_ident, quote},
    syn::parse_quote,
};

pub struct ArgumentsGenerator;

impl ArgumentsGenerator {
    pub fn new() -> Self {
        ArgumentsGenerator
    }
}

impl StagedGenerator for ArgumentsGenerator {
    fn append(&mut self, context: &mut GenerationContext) -> Result<(), syn::Error> {
        let Some(ref args) = context.input.args else {
            return Ok(());
        };

        let context_name = &context.input.item_struct.ident;
        let (name, args_struct) = match args {
            Arguments::Struct(name) => (name.clone(), None),
            Arguments::Values(args) => {
                let struct_name = format_ident!("{context_name}Args");
                let fields = args
                    .iter()
                    .map(|Argument { name, ty }: &Argument| quote!(pub #name: #ty));

                let generated_struct = quote! {
                    #[derive(Debug, PartialEq, bytemuck::AnyBitPattern, bytemuck::NoUninit, Copy, Clone)]
                    #[repr(C)]
                    pub struct #struct_name {
                        #(#fields),*
                    }
                };

                (struct_name, Some(generated_struct))
            }
        };

        context
            .generated_results
            .new_fields
            .push(parse_quote!(pub args: Args<'info, #name>));
        context.generated_results.inside.extend(
            quote!(let args = Args::<#name>::from_entrypoint(accounts, instruction_data)?;),
        );

        if let Some(args_struct) = args_struct {
            context.generated_results.outside.extend(args_struct);
        }

        Ok(())
    }
}

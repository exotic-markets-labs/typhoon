use {
    super::{ConstraintGenerator, GeneratorResult},
    crate::{
        arguments::{Argument, Arguments},
        context::Context,
        visitor::ContextVisitor,
    },
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote},
    syn::{parse_quote, Ident},
};

#[derive(Default)]
pub struct ArgumentsGenerator {
    context_name: Option<Ident>,
    arg_name: Option<Ident>,
    arg_struct: Option<TokenStream>,
}

impl ArgumentsGenerator {
    pub fn new() -> Self {
        ArgumentsGenerator::default()
    }
}

impl ConstraintGenerator for ArgumentsGenerator {
    fn generate(&self) -> Result<GeneratorResult, syn::Error> {
        let mut result = GeneratorResult::default();
        if let Some(ref name) = self.arg_name {
            result
                .new_fields
                .push(parse_quote!(pub args: Args<'info, #name>));
            result.at_init =
                quote!(let args = Args::<#name>::from_entrypoint(accounts, instruction_data)?;);

            if let Some(ref arg_struct) = self.arg_struct {
                result.global_outside = arg_struct.clone();
            }
        }
        Ok(result)
    }
}

impl ContextVisitor for ArgumentsGenerator {
    fn visit_context(&mut self, context: &Context) -> Result<(), syn::Error> {
        self.context_name = Some(context.item_struct.ident.clone());

        self.visit_accounts(&context.accounts)?;

        if let Some(args) = &context.args {
            self.visit_arguments(args)?;
        }

        Ok(())
    }

    fn visit_arguments(&mut self, arguments: &Arguments) -> Result<(), syn::Error> {
        match arguments {
            Arguments::Struct(name) => self.arg_name = Some(name.clone()),
            Arguments::Values(args) => {
                let Some(ref context_name) = self.context_name else {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "Not in a valid context.",
                    ));
                };

                let struct_name = format_ident!("{context_name}Args");
                let fields = args
                    .iter()
                    .map(|Argument { name, ty }: &Argument| quote!(pub #name: #ty));

                let generated_struct = quote! {
                    #[derive(Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
                    #[repr(C)]
                    pub struct #struct_name {
                        #(#fields),*
                    }
                };

                self.arg_name = Some(struct_name);
                self.arg_struct = Some(generated_struct);
            }
        }

        Ok(())
    }
}

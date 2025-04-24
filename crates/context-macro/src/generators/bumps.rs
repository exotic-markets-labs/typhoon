use {
    super::tokens_gen::BumpTokenGenerator,
    crate::{constraints::Constraint, visitor::ContextVisitor, GenerationContext, StagedGenerator},
    quote::{format_ident, quote},
    syn::parse_quote,
};

pub struct BumpsGenerator;

impl BumpsGenerator {
    pub fn new() -> Self {
        BumpsGenerator
    }
}

impl StagedGenerator for BumpsGenerator {
    fn append(&mut self, context: &mut GenerationContext) -> Result<(), syn::Error> {
        let context_name = &context.input.item_struct.ident;
        let mut fields = Vec::new();

        for account in &context.input.accounts {
            // let has_init_if_needed = account
            //     .constraints
            //     .0
            //     .iter()
            //     .any(|c| matches!(c, Constraint::InitIfNeeded(_)));
            let has_bump = account
                .constraints
                .0
                .iter()
                .any(|c| matches!(c, Constraint::Bump(_)));

            if has_bump {
                let mut pda_generator = BumpTokenGenerator::new(account);
                pda_generator.visit_account(account)?;

                let (pda, _, check, is_field_generated) = pda_generator.generate()?;

                if is_field_generated {
                    fields.push(account.name.clone());
                }

                context.generated_results.inside.extend(quote! {
                    #pda
                    #check
                });
            }
        }

        if !fields.is_empty() {
            let struct_name = format_ident!("{}Bumps", context_name);
            let struct_fields = &fields;
            let bumps_struct = quote! {
                #[derive(Debug, PartialEq)]
                pub struct #struct_name {
                    #(pub #struct_fields: u8,)*
                }
            };

            context.generated_results.outside.extend(bumps_struct);
            let assign_fields = fields.iter().map(|n| {
                let bump_ident = format_ident!("{}_bump", n);
                quote!(#n: #bump_ident)
            });
            context.generated_results.inside.extend(quote! {
                let bumps = #struct_name {
                    #(#assign_fields),*
                };
            });

            context.generated_results.new_fields.push(parse_quote! {
                pub bumps: #struct_name
            });
        }

        Ok(())
    }
}

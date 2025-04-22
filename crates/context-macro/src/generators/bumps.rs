use {
    super::tokens_gen::BumpTokenGenerator,
    crate::{
        constraints::ConstraintBump, visitor::ContextVisitor, GenerationContext, StagedGenerator,
    },
    quote::{format_ident, quote},
    syn::parse_quote,
};

#[derive(Default)]
pub struct BumpsGenerator {
    is_pda: bool,
}

impl BumpsGenerator {
    pub fn new() -> Self {
        BumpsGenerator::default()
    }
}

impl ContextVisitor for BumpsGenerator {
    fn visit_bump(&mut self, _constraint: &ConstraintBump) -> Result<(), syn::Error> {
        self.is_pda = true;

        Ok(())
    }
}

impl StagedGenerator for BumpsGenerator {
    fn append(&mut self, context: &mut GenerationContext) -> Result<(), syn::Error> {
        let context_name = &context.input.item_struct.ident;
        let mut fields = Vec::new();

        for account in &context.input.accounts {
            self.visit_constraints(&account.constraints)?;
            let mut pda_generator = BumpTokenGenerator::new(account);
            pda_generator.visit_account(account)?;

            if self.is_pda {
                let (pda, _, check, is_field_generated) = pda_generator.generate()?;

                if is_field_generated {
                    fields.push(account.name.clone());
                }

                context.generated_results.inside.extend(quote! {
                    #pda
                    #check
                });
                self.is_pda = false;
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

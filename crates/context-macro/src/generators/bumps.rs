use {
    super::tokens_gen::{BumpTokenGenerator, InitTokenGenerator},
    crate::{
        constraints::{ConstraintBump, ConstraintInit, ConstraintInitIfNeeded},
        visitor::ContextVisitor,
        GenerationContext, StagedGenerator,
    },
    quote::{format_ident, quote},
    syn::{parse_quote, Ident},
};

#[derive(Default)]
struct Checks {
    has_bump: bool,
    has_init_if_needed: bool,
    has_init: bool,
}

impl Checks {
    pub fn new() -> Self {
        Checks::default()
    }
}

impl ContextVisitor for Checks {
    fn visit_init_if_needed(
        &mut self,
        _constraint: &ConstraintInitIfNeeded,
    ) -> Result<(), syn::Error> {
        self.has_init_if_needed = true;
        Ok(())
    }

    fn visit_bump(&mut self, _constraint: &ConstraintBump) -> Result<(), syn::Error> {
        self.has_bump = true;
        Ok(())
    }

    fn visit_init(&mut self, _constraint: &ConstraintInit) -> Result<(), syn::Error> {
        self.has_init = true;
        Ok(())
    }
}

pub struct BumpsGenerator;

impl BumpsGenerator {
    pub fn new() -> Self {
        BumpsGenerator
    }
}

impl BumpsGenerator {
    fn append_field(&mut self, context: &mut GenerationContext, fields: Vec<Ident>) {
        let context_name = &context.input.item_struct.ident;
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
}

impl StagedGenerator for BumpsGenerator {
    fn append(&mut self, context: &mut GenerationContext) -> Result<(), syn::Error> {
        let mut fields = Vec::new();

        for account in &context.input.accounts {
            let mut checks = Checks::new();
            checks.visit_account(account)?;

            let name = &account.name;
            let account_ty = &account.ty;

            if checks.has_init_if_needed {
                let is_initialized_name = format_ident!("{}_is_initialized", name);
                let mut init_gen = InitTokenGenerator::new(account);
                init_gen.visit_account(account)?;
                let init_token = init_gen.generate()?;

                if checks.has_bump {
                    let pda_key = format_ident!("{}_key", name);
                    let pda_bump = format_ident!("{}_bump", name);
                    let mut bump_gen = BumpTokenGenerator::new(account);
                    bump_gen.visit_account(account)?;
                    let (pda_token, find_pda_token, check_token, is_field_generated) =
                        bump_gen.generate()?;

                    if is_field_generated {
                        fields.push(account.name.clone());
                    }

                    context.generated_results.inside.extend(quote! {
                        let #is_initialized_name = <Mut<UncheckedAccount> as ChecksExt>::is_initialized(&#name);
                        let (#name, #pda_key, #pda_bump) = if #is_initialized_name {
                            let #name = <#account_ty as FromAccountInfo>::try_from_info(#name.into())?;
                            #pda_token
                            (#name, #pda_key, #pda_bump)
                        }else {
                            #find_pda_token
                            let #name = { #init_token };
                            (#name, #pda_key, #pda_bump)
                        };
                        #check_token
                    });
                } else {
                    context.generated_results.inside.extend(quote! {
                        let #is_initialized_name = <Mut<UncheckedAccount> as ChecksExt>::is_initialized(&#name);
                        let #name = if #is_initialized_name {
                            <#account_ty as FromAccountInfo>::try_from_info(#name.into())?
                        }else {
                            #init_token
                        };
                });
                }
            } else {
                if checks.has_bump {
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

                if checks.has_init {
                    let mut init_gen = InitTokenGenerator::new(account);
                    init_gen.visit_account(account)?;
                    let init_token = init_gen.generate()?;

                    context.generated_results.inside.extend(quote! {
                        let #name: #account_ty = {
                            #init_token
                        };
                    });
                }
            }
        }

        if !fields.is_empty() {
            self.append_field(context, fields);
        }

        Ok(())
    }
}

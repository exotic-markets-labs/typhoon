use {
    super::{
        tokens_gen::{BumpTokenGenerator, InitTokenGenerator},
        GeneratorResult,
    },
    crate::{
        constraints::{ConstraintBump, ConstraintInit, ConstraintInitIfNeeded},
        context::Context,
        visitor::ContextVisitor,
        StagedGenerator,
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

pub struct BumpsGenerator<'a>(&'a Context);

impl<'a> BumpsGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        BumpsGenerator(context)
    }
}

impl BumpsGenerator<'_> {
    fn append_field(&mut self, result: &mut GeneratorResult, fields: Vec<Ident>) {
        let context_name = &self.0.item_struct.ident;
        let struct_name = format_ident!("{}Bumps", context_name);
        let struct_fields = &fields;
        let bumps_struct = quote! {
            #[derive(Debug, PartialEq)]
            pub struct #struct_name {
                #(pub #struct_fields: u8,)*
            }
        };

        result.outside.extend(bumps_struct);
        let assign_fields = fields.iter().map(|n| {
            let bump_ident = format_ident!("{}_bump", n);
            quote!(#n: #bump_ident)
        });
        result.inside.extend(quote! {
            let bumps = #struct_name {
                #(#assign_fields),*
            };
        });

        result.new_fields.push(parse_quote! {
            pub bumps: #struct_name
        });
    }
}

impl StagedGenerator for BumpsGenerator<'_> {
    fn append(&mut self, result: &mut GeneratorResult) -> Result<(), syn::Error> {
        let mut fields = Vec::new();

        for account in &self.0.accounts {
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

                    result.inside.extend(quote! {
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
                    result.inside.extend(quote! {
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

                    result.inside.extend(quote! {
                        #pda
                        #check
                    });
                }

                if checks.has_init {
                    let mut init_gen = InitTokenGenerator::new(account);
                    init_gen.visit_account(account)?;
                    let init_token = init_gen.generate()?;

                    result.inside.extend(quote! {
                        let #name: #account_ty = {
                            #init_token
                        };
                    });
                }
            }
        }

        if !fields.is_empty() {
            self.append_field(result, fields);
        }

        Ok(())
    }
}

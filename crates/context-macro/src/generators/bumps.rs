use {
    super::GeneratorResult,
    crate::{
        constraints::{ConstraintBump, ConstraintInitIfNeeded},
        context::Context,
        visitor::ContextVisitor,
        StagedGenerator,
    },
    quote::{format_ident, quote},
    syn::{parse_quote, Ident},
};

#[derive(Default)]
struct Checks {
    is_pda: bool,
    has_bump: bool,
    has_init_if_needed: bool,
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

    fn visit_bump(&mut self, constraint: &ConstraintBump) -> Result<(), syn::Error> {
        self.is_pda = true;
        self.has_bump = constraint.0.is_some();
        Ok(())
    }
}

pub struct BumpsGenerator<'a>(&'a Context);

impl<'a> BumpsGenerator<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self(context)
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

            if checks.is_pda && (!checks.has_bump || checks.has_init_if_needed) {
                fields.push(account.name.clone());
            }
        }

        if !fields.is_empty() {
            self.append_field(result, fields);
        }

        Ok(())
    }
}

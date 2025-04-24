use {
    crate::{
        constraints::Constraint, generators::tokens_gen::InitTokenGenerator,
        visitor::ContextVisitor, StagedGenerator,
    },
    quote::{format_ident, quote},
};

pub struct InitIfNeededGenerator;

impl InitIfNeededGenerator {
    pub fn new() -> Self {
        InitIfNeededGenerator
    }
}

impl StagedGenerator for InitIfNeededGenerator {
    fn append(&mut self, context: &mut crate::GenerationContext) -> Result<(), syn::Error> {
        let accs = context.input.accounts.iter().filter(|acc| {
            acc.constraints
                .0
                .iter()
                .any(|c| matches!(c, Constraint::InitIfNeeded(_)))
        });

        for acc in accs {
            let mut init_gen = InitTokenGenerator::new(acc);
            init_gen.visit_account(acc)?;
            let init_token = init_gen.generate()?;

            let name = &acc.name;
            // let pda_key = format_ident!("{}_key", name);
            // let pda_bump = format_ident!("{}_bump", name);
            let account_ty = &acc.ty;
            let is_initialized_name = format_ident!("{}_is_initialized", name);

            // let maybe_bump_token = if acc
            //     .constraints
            //     .0
            //     .iter()
            //     .any(|c| matches!(c, Constraint::Bump(_)))
            // {
            //     let mut bump_gen = BumpTokenGenerator::new(acc);
            //     bump_gen.visit_account(acc)?;
            //     let (pda_token, find_pda_token, check_token, _) = bump_gen.generate()?;
            //     Some(quote! {
            //         let (#pda_key, #pda_bump) = if #is_initialized_name {
            //             #pda_token
            //             (#pda_key, #pda_bump)
            //         } else {
            //             #find_pda_token
            //         };
            //         #check_token
            //     })
            // } else {
            //     None
            // };

            context.generated_results.inside.extend(quote! {
                let #is_initialized_name = <Mut<UncheckedAccount> as ChecksExt>::is_initialized(&#name);
                let #name = if #is_initialized_name {
                    <#account_ty as FromAccountInfo>::try_from_info(#name.into())?
                }else {
                    #init_token
                };
            });
        }

        Ok(())
    }
}

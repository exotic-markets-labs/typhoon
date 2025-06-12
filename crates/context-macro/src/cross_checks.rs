use {crate::context::Context, syn::spanned::Spanned, typhoon_syn::constraints::Constraint};

fn check_program_prerequisite(context: &Context, program: &str) -> Result<(), syn::Error> {
    let has_system_program = context.accounts.iter().any(|acc| {
        (acc.ty.ident == "Program" || acc.ty.ident == "Interface") && acc.inner_ty == program
    });

    if !has_system_program {
        return Err(syn::Error::new(
            context.item_struct.span(),
            format!("One constraint requires including the `Program<{program}>` account."),
        ));
    }

    Ok(())
}

pub fn cross_checks(context: &Context) -> Result<(), syn::Error> {
    for acc in &context.accounts {
        if acc
            .constraints
            .0
            .iter()
            .any(|c| matches!(c, Constraint::Init(_) | Constraint::InitIfNeeded(_)))
        {
            check_program_prerequisite(context, "System")?;

            if acc.inner_ty == "Mint" || acc.inner_ty == "TokenAccount" {
                check_program_prerequisite(context, "TokenProgram")?;

                if acc
                    .constraints
                    .0
                    .iter()
                    .any(|c| matches!(c, Constraint::AssociatedToken(_)))
                {
                    check_program_prerequisite(context, "AtaTokenProgram")?;
                }
            }
        }
    }

    Ok(())
}

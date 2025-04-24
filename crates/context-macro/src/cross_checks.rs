use {
    crate::{constraints::Constraint, GenerationContext},
    syn::spanned::Spanned,
};

fn check_program_prerequisite(
    context: &GenerationContext,
    program: &str,
) -> Result<(), syn::Error> {
    let has_system_program = context
        .input
        .accounts
        .iter()
        .any(|acc| acc.ty.ident == "Program" && acc.inner_ty == program);

    if !has_system_program {
        return Err(syn::Error::new(
            context.input.item_struct.span(),
            format!("One constraint requires including the `Program<{program}>` account."),
        ));
    }

    Ok(())
}

pub fn cross_checks(context: &GenerationContext) -> Result<(), syn::Error> {
    for acc in &context.input.accounts {
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

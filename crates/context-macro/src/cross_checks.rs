// use {crate::GenerationContext, syn::spanned::Spanned};

// fn check_prerequisite(context: &GenerationContext, program: &str) -> Result<(), syn::Error> {
//     let has_system_program = context
//         .input
//         .accounts
//         .iter()
//         .any(|acc| acc.ty.ident == "Program" && acc.inner_ty == program);

//     if !has_system_program {
//         return Err(syn::Error::new(
//             context.input.item_struct.span(),
//             format!(
//                 "One constraint requires including the `Program<{}>` account",
//                 program
//             ),
//         ));
//     }

//     Ok(())
// }

// pub fn cross_checks(context: &GenerationContext) {
//     // for acc in context.input.accounts {
//     //     // if acc.constraints.0.iter().any
//     // }
// }

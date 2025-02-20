use {
    crate::anchor::{gen_accounts, gen_instructions, program_id::gen_program_id},
    anchor_lang_idl_spec::Idl,
    quote::{format_ident, quote},
};

pub fn gen_cpi(idl: &Idl) -> proc_macro2::TokenStream {
    let name = format_ident!("{}_cpi", idl.metadata.name);
    let program_id = gen_program_id(idl);
    let accounts = gen_accounts(idl);
    let instructions = gen_instructions(idl);

    quote! {
        pub mod #name {
            use ::typhoon::prelude::*;

            #program_id
            #accounts
            #instructions
        }
    }
}

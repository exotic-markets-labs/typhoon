use {
    crate::{
        anchor::{gen_accounts, gen_instructions, program_id::gen_program_id},
        idl::Idl,
    },
    quote::{format_ident, quote},
};

pub fn gen_cpi(idl: &Idl) -> proc_macro2::TokenStream {
    let name = idl
        .name
        .as_ref()
        .or(idl.metadata.name.as_ref())
        .unwrap_or_else(|| panic!("IDL is missing name"));
    let address = idl
        .address
        .as_ref()
        .or(idl.metadata.address.as_ref())
        .unwrap_or_else(|| panic!("IDL is missing address"));
    let mod_name = format_ident!("{name}_cpi");
    let program_id = gen_program_id(name, address);
    let accounts = gen_accounts(&idl.accounts, &idl.types);
    let instructions = gen_instructions(&idl.instructions);

    quote! {
        pub mod #mod_name {
            use super::*;

            #program_id
            #accounts
            #instructions
        }
    }
}

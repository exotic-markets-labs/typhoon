use {
    heck::ToUpperCamelCase,
    quote::{format_ident, quote},
};

pub fn gen_program_id(name: &str, address: &str) -> proc_macro2::TokenStream {
    let name = &name.to_upper_camel_case();
    let ident = format_ident!("{name}Program");

    quote! {
        pub const PROGRAM_ID: Address = Address::from_str_const(#address);

        pub struct #ident;

        impl CheckProgramId for #ident {
            #[inline(always)]
            fn address_eq(program_id: &Address) -> bool {
                address_eq(program_id, &PROGRAM_ID)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_program_id() {
        let generated =
            gen_program_id("test", "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").to_string();
        let expected = quote! {
            pub const PROGRAM_ID: Address = Address::from_str_const("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

            pub struct TestProgram;

            impl ProgramId for TestProgram {
                const ID: Address = PROGRAM_ID;
            }
        }
        .to_string();

        assert_eq!(generated, expected);
    }
}

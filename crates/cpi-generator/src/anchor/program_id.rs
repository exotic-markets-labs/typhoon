use {
    five8_const::decode_32_const,
    heck::ToUpperCamelCase,
    quote::{format_ident, quote},
};

pub fn gen_program_id(name: &str, address: &str) -> proc_macro2::TokenStream {
    let name = &name.to_upper_camel_case();
    let ident = format_ident!("{name}Program");
    let id_array = decode_32_const(address);

    quote! {
        pub const PROGRAM_ID: Pubkey = [#(#id_array),*];

        pub struct #ident;

        impl ProgramId for #ident {
            const ID: Pubkey = PROGRAM_ID;
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
            pub struct TestProgram;

            impl ProgramId for TestProgram {
                const ID: Pubkey = [218u8 , 7u8 , 92u8 , 178u8 , 255u8 , 94u8 , 198u8 , 129u8 , 118u8 , 19u8 , 222u8 , 83u8 , 11u8 , 105u8 , 42u8 , 135u8 , 53u8 , 71u8 , 119u8 , 105u8 , 218u8 , 71u8 , 67u8 , 12u8 , 189u8 , 129u8 , 84u8 , 51u8 , 92u8 , 74u8 , 131u8 , 39u8];
            }
        }.to_string();

        assert_eq!(generated, expected);
    }
}

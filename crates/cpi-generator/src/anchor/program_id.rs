use {
    anchor_lang_idl_spec::Idl, five8_const::decode_32_const, heck::ToUpperCamelCase,
    proc_macro2::Span, quote::quote, syn::Ident,
};

pub fn gen_program_id(idl: &Idl) -> proc_macro2::TokenStream {
    let name = &idl.metadata.name.to_upper_camel_case();
    let ident = Ident::new(&format!("{name}Program"), Span::call_site());
    let program_id = &idl.address;
    let id_array = decode_32_const(program_id);

    quote! {
        pub struct #ident;

        impl ProgramId for #ident {
            const ID: program::pubkey::Pubkey = program::pubkey_from_array([#(#id_array),*]);
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, anchor_lang_idl_spec::IdlMetadata};

    #[test]
    fn test_gen_program_id() {
        let idl = Idl {
            metadata: IdlMetadata {
                name: "test".to_string(),
                dependencies: vec![],
                contact: None,
                deployments: None,
                description: None,
                repository: None,
                spec: "0.1.0".to_string(),
                version: "0.1.0".to_string(),
            },
            address: "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS".to_string(),
            accounts: vec![],
            constants: vec![],
            docs: vec![],
            errors: vec![],
            events: vec![],
            instructions: vec![],
            types: vec![],
        };

        let generated = gen_program_id(&idl).to_string();
        let expected = quote! {
            pub struct TestProgram;

            impl ProgramId for TestProgram {
                const ID: program::pubkey::Pubkey = program::pubkey_from_array([218u8 , 7u8 , 92u8 , 178u8 , 255u8 , 94u8 , 198u8 , 129u8 , 118u8 , 19u8 , 222u8 , 83u8 , 11u8 , 105u8 , 42u8 , 135u8 , 53u8 , 71u8 , 119u8 , 105u8 , 218u8 , 71u8 , 67u8 , 12u8 , 189u8 , 129u8 , 84u8 , 51u8 , 92u8 , 74u8 , 131u8 , 39u8]);
            }
        }.to_string();

        assert_eq!(generated, expected);
    }
}

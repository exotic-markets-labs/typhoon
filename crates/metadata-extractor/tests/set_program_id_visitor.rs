use {
    codama::{CrateKorok, CrateStore, KorokVisitable, Node},
    quote::quote,
    typhoon_metadata_extractor::visitors::SetProgramIdVisitor,
};

#[test]
fn it_gets_program_ids_from_the_declare_id_macro() {
    let store = CrateStore::hydrate(quote! {
        program_id!("MyProgramAddress1111111111111111111111111");
    })
    .unwrap();
    let mut korok = CrateKorok::parse(&store).unwrap();
    korok.accept(&mut SetProgramIdVisitor::new()).unwrap();

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };
    assert_eq!(
        program.public_key,
        "MyProgramAddress1111111111111111111111111"
    );
}

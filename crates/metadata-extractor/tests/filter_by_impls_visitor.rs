use {
    codama_attributes::Attributes,
    codama_korok_visitors::KorokVisitable,
    codama_koroks::{CrateKorok, ItemKorok},
    codama_stores::CrateStore,
    std::path::PathBuf,
    syn::{parse_quote, File},
    typhoon_metadata_extractor::visitors::FilterByImplsVisitor,
};

#[test]
fn it_apply_marker_node() {
    let parsed: File = parse_quote! {
        pub struct RandomState {}

        impl Owner for RandomState {
            const OWNER: Pubkey = crate::ID;
        }
    };
    let another_file = parsed.clone();
    let items = ItemKorok::parse_all(&another_file.items, &[], &mut 0).unwrap();

    let mut korok = CrateKorok {
        attributes: Attributes(vec![]),
        items,
        node: None,
        store: &CrateStore {
            file: parsed,
            manifest: None,
            file_modules: vec![],
            path: PathBuf::new(),
        },
    };

    let mut visitor = FilterByImplsVisitor::new(&["Owner"]);
    korok.accept(&mut visitor);

    println!("{visitor:?}")
    // println!("{korok:?}");
    // assert!(korok.iter().len() == 1)

    // korok.acc
}

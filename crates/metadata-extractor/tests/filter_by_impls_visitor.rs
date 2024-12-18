use std::path::PathBuf;

use codama_attributes::Attributes;
use codama_korok_visitors::KorokVisitable;
use codama_koroks::{CrateKorok, ItemKorok};
use codama_stores::CrateStore;
use crayfish_metadata_extractor::visitors::FilterByImplsVisitor;
use syn::{parse_quote, File};

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

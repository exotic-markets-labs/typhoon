use {
    codama_korok_visitors::KorokVisitable,
    codama_koroks::ItemKorok,
    syn::{parse_quote, Item},
    typhoon_metadata_extractor::visitors::CacheByImplsVisitor,
};

#[test]
fn it_apply_marker_node() {
    let ast: Item = parse_quote! {
        mod program {
            pub struct RandomState {}

            impl Owner for RandomState {
                const OWNER: Pubkey = crate::ID;
            }
        }
    };
    let mut korok = ItemKorok::parse(&ast, &[], &mut 0).unwrap();
    let mut visitor = CacheByImplsVisitor::new(&["Owner"]);

    korok.accept(&mut visitor);

    let ItemKorok::Module(module) = &korok else {
        panic!("Expected program module");
    };

    let ItemKorok::Struct(_) = &module.items[0] else {
        panic!("Expected RandomState struct");
    };

    let cache = visitor.get_cache();
    assert_eq!(&cache, &["RandomState"]);
}

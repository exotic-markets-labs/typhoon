use {
    codama::{KorokTrait, KorokVisitor, PublicKeyTypeNode, UniformVisitor},
    codama_korok_visitors::KorokVisitable,
    codama_koroks::ItemKorok,
    syn::{parse_quote, Item},
    typhoon_metadata_extractor::visitors::FilterByImplsVisitor,
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

    korok.accept(&mut FilterByImplsVisitor::new(
        &["Owner"],
        UniformVisitor::new(|mut k, visitor| {
            visitor.visit_children(&mut k);
            k.set_node(Some(PublicKeyTypeNode::new().into()));
        }),
    ));

    let ItemKorok::Module(module) = &korok else {
        panic!("Expected program module");
    };

    let ItemKorok::Struct(random_state) = &module.items[0] else {
        panic!("Expected RandomState struct");
    };

    assert_eq!(random_state.node, Some(PublicKeyTypeNode::new().into()));
}

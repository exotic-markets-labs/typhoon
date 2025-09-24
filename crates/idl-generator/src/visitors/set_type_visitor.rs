use {
    codama::{CodamaResult, KorokVisitor},
    codama_nodes::{
        ArrayTypeNode, BooleanTypeNode, FixedCountNode, MapTypeNode, NumberFormat::*,
        NumberTypeNode, OptionTypeNode, PrefixedCountNode, PublicKeyTypeNode, SetTypeNode,
        SizePrefixTypeNode, StringTypeNode, TypeNode,
    },
    codama_syn_helpers::extensions::*,
};

#[derive(Default)]
pub struct SetBorshTypesVisitor;

impl SetBorshTypesVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for SetBorshTypesVisitor {
    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        if korok.node.is_some() {
            return Ok(());
        }
        if let Some(node) = get_type_node(&korok.ast.ty) {
            korok.set_type_node(node);
        }
        Ok(())
    }
}

pub fn get_type_node(ty: &syn::Type) -> Option<TypeNode> {
    match ty {
        syn::Type::Path(syn::TypePath { path, .. }) => {
            if path.leading_colon.is_some() {
                return None;
            }
            match (
                // a::b<B>::c::HashMap<K, V> -> a::b::c
                path.prefix().as_str(),
                // a::b::c::HashMap<K, V> -> HashMap
                path.last_str().as_str(),
                // a::b::c::HashMap<K, V> -> [K, V]
                path.generic_types().as_slice(),
            ) {
                ("" | "std::primitive", "bool", []) => Some(BooleanTypeNode::default().into()),
                ("" | "std::primitive", "usize", []) => Some(NumberTypeNode::le(U64).into()),
                ("" | "std::primitive", "u8", []) => Some(NumberTypeNode::le(U8).into()),
                ("" | "std::primitive", "u16", []) => Some(NumberTypeNode::le(U16).into()),
                ("" | "std::primitive", "u32", []) => Some(NumberTypeNode::le(U32).into()),
                ("" | "std::primitive", "u64", []) => Some(NumberTypeNode::le(U64).into()),
                ("" | "std::primitive", "u128", []) => Some(NumberTypeNode::le(U128).into()),
                ("" | "std::primitive", "isize", []) => Some(NumberTypeNode::le(I64).into()),
                ("" | "std::primitive", "i8", []) => Some(NumberTypeNode::le(I8).into()),
                ("" | "std::primitive", "i16", []) => Some(NumberTypeNode::le(I16).into()),
                ("" | "std::primitive", "i32", []) => Some(NumberTypeNode::le(I32).into()),
                ("" | "std::primitive", "i64", []) => Some(NumberTypeNode::le(I64).into()),
                ("" | "std::primitive", "i128", []) => Some(NumberTypeNode::le(I128).into()),
                ("" | "std::primitive", "f32", []) => Some(NumberTypeNode::le(F32).into()),
                ("" | "std::primitive", "f64", []) => Some(NumberTypeNode::le(F64).into()),
                (_, "ShortU16", []) => Some(NumberTypeNode::le(ShortU16).into()),
                ("" | "solana_sdk::pubkey" | "solana_program" | "solana_pubkey", "Pubkey", []) => {
                    Some(PublicKeyTypeNode::new().into())
                }
                ("" | "std::string", "String", []) => Some(
                    SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into(),
                ),
                ("" | "std::option", "Option", [t]) => {
                    get_type_node(t).map(|item| OptionTypeNode::new(item).into())
                }
                ("" | "std::vec", "Vec", [t]) => get_type_node(t).map(|item| {
                    ArrayTypeNode::new(item, PrefixedCountNode::new(NumberTypeNode::le(U32))).into()
                }),
                ("" | "std::collections", "HashSet" | "BTreeSet", [t]) => {
                    get_type_node(t).map(|item| {
                        SetTypeNode::new(item, PrefixedCountNode::new(NumberTypeNode::le(U32)))
                            .into()
                    })
                }
                ("" | "std::collections", "HashMap" | "BTreeMap", [k, v]) => {
                    match (get_type_node(k), get_type_node(v)) {
                        (Some(key), Some(value)) => Some(
                            MapTypeNode::new(
                                key,
                                value,
                                PrefixedCountNode::new(NumberTypeNode::le(U32)),
                            )
                            .into(),
                        ),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        syn::Type::Array(syn::TypeArray { elem, len, .. }) => {
            let Ok(size) = len.as_unsigned_integer::<usize>() else {
                return None;
            };
            get_type_node(elem)
                .map(|item| ArrayTypeNode::new(item, FixedCountNode::new(size)).into())
        }
        _ => None,
    }
}

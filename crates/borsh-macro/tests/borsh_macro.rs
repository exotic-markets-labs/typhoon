use {borsh::BorshSerialize, pinocchio::pubkey::Pubkey, typhoon_borsh_macro::borsh};

#[borsh]
pub struct ChildStruct {
    pub element_1: i64,
    pub element_2: i128,
    pub element_3: i8,
    pub element_4: bool,
    pub element_5: Pubkey,
}

#[borsh]
pub struct Struct {
    pub element_1: u64,
    pub element_2: u16,
    pub element_3: String,
    pub element_4: u8,
    pub element_5: ChildStruct,
    pub element_6: Vec<u32>,
}

#[borsh]
pub struct ComplexType {
    pub element1: Vec<String>,
    pub element2: [u32; 4],
}

#[test]
fn test_complex_type() {
    let mut buffer = Vec::new();
    let test_struct = ComplexTypeTest {
        element1: vec!["Random".to_string(), "Random2".to_string()],
        element2: [1, 2, 3, 4],
    };
    test_struct.serialize(&mut buffer).unwrap();
    let struct_test: &ComplexType = unsafe { core::mem::transmute(buffer.as_slice()) };
    assert_eq!(struct_test.total_len(), buffer.len());
    let elements = struct_test.element1();
    assert_eq!(elements.len(), 2);
    assert_eq!(struct_test.element2(), [1, 2, 3, 4]);
}

#[test]
fn test_gen() {
    let mut buffer = Vec::new();
    let test_struct = StructTest {
        element_1: 3,
        element_2: 2,
        element_3: "1".to_string(),
        element_4: 0,
        element_5: ChildStructTest {
            element_1: 10,
            element_2: 20,
            element_3: -3,
            element_4: true,
            element_5: Pubkey::default(),
        },
        element_6: vec![1, 2, 3],
    };
    test_struct.serialize(&mut buffer).unwrap();

    let struct_test: &Struct = unsafe { core::mem::transmute(buffer.as_slice()) };
    assert_eq!(struct_test.element_1_offset(), 0);
    assert_eq!(struct_test.element_2_offset(), 8);
    assert_eq!(struct_test.element_3_offset(), 10);
    assert_eq!(struct_test.element_4_offset(), 15);
    assert_eq!(struct_test.element_5_offset(), 16);
    assert_eq!(struct_test.element_6_offset(), 74);
    assert_eq!(struct_test.total_len(), buffer.len());

    assert_eq!(struct_test.element_1(), 3);
    assert_eq!(struct_test.element_2(), 2);
    assert_eq!(struct_test.element_3(), "1");
    assert_eq!(struct_test.element_4(), 0);
    assert_eq!(struct_test.element_5().element_1(), 10);
    assert_eq!(struct_test.element_5().element_2(), 20);
    assert_eq!(struct_test.element_5().element_3(), -3);
    assert!(struct_test.element_5().element_4());
    assert_eq!(struct_test.element_5().element_5(), Pubkey::default());
    assert_eq!(struct_test.element_6(), &[1, 2, 3]);
}

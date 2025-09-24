use bytemuck::{AnyBitPattern, NoUninit};

#[derive(AnyBitPattern, NoUninit, Clone, Copy)]
#[repr(C, packed)]
pub struct MakeArgs {
    pub seed: u8,
    pub receive: u64,
    pub amount: u64,
}

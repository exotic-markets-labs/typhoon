use typhoon::prelude::*;
use typhoon_program::pubkey::Pubkey;

pub const ID: Pubkey = [
    218, 7, 92, 178, 255, 94, 198, 129, 118, 19, 222, 83, 11, 105, 42, 135, 53, 71, 119, 105, 218,
    71, 67, 12, 189, 129, 84, 51, 92, 74, 131, 39,
];

#[account]
pub struct TestState {
    pub foo: u64,
}

#[account]
pub struct TestAnotherState {
    #[key]
    pub foo: ZCPubkey,
}

pub fn main() {}

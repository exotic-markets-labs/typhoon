use {
    bytemuck::{AnyBitPattern, NoUninit},
    typhoon::prelude::*,
};

#[derive(AccountState, NoUninit, AnyBitPattern, Clone, Copy)]
#[repr(C)]
pub struct RandomData {
    pub counter: u8,
}

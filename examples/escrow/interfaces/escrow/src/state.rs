use {
    bytemuck::{AnyBitPattern, NoUninit},
    typhoon::prelude::*,
};

#[derive(AccountState, NoUninit, AnyBitPattern, Clone, Copy)]
#[repr(C, packed)]
pub struct Escrow {
    pub maker: Pubkey,  // Creator of the escrow
    pub mint_a: Pubkey, // Token being deposited
    pub mint_b: Pubkey, // Token being requested
    pub seed: u8,       // Random seed for PDA derivation
    pub receive: u64,   // Amount of token B wanted
    pub bump: u8,       // PDA bump seed
}

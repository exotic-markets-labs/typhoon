use solana_address::Address;
use typhoon::prelude::*;

pub const RANDOM_PDA: (Address, u8) = find_program_address_const(&[b"random"], &crate::ID);

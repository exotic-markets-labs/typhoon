use const_crypto::ed25519;
use typhoon::prelude::Pubkey;

pub const RANDOM_PDA: (Pubkey, u8) = ed25519::derive_program_address(&[b"random"], &crate::ID);

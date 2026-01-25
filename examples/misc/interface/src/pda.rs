use const_crypto::ed25519;
use solana_address::Address;

pub const RANDOM_PDA: (Address, u8) = {
    let (key, bump) = ed25519::derive_program_address(&[b"random"], crate::ID.as_array());
    (Address::new_from_array(key), bump)
};

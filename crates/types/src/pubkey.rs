use {
    std::ops::Deref,
    typhoon_program::{pubkey::Pubkey, pubkey_from_array, pubkey_to_bytes},
    zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout},
};

#[derive(KnownLayout, Immutable, FromBytes, IntoBytes, Debug, Clone, Copy, PartialEq)]
pub struct ZCPubkey([u8; 32]);

impl ZCPubkey {
    pub fn new(value: [u8; 32]) -> Self {
        ZCPubkey(value)
    }
}

impl Deref for ZCPubkey {
    type Target = Pubkey;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.0) }
    }
}

impl From<ZCPubkey> for Pubkey {
    fn from(value: ZCPubkey) -> Self {
        pubkey_from_array(value.0)
    }
}

impl From<Pubkey> for ZCPubkey {
    fn from(value: Pubkey) -> Self {
        ZCPubkey(pubkey_to_bytes(value))
    }
}

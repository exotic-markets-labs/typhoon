pub use pinocchio_token::instructions::*;
use {
    pinocchio_token::{
        state::{Mint as SplMint, TokenAccount as SplTokenAccount},
        ID,
    },
    std::{mem::transmute, ops::Deref},
    typhoon_program::pubkey::Pubkey,
    typhoon_traits::{Discriminator, Owner, RefFromBytes},
};

#[repr(transparent)]
pub struct Mint(SplMint);

impl RefFromBytes for Mint {
    fn read(data: &[u8]) -> Option<&Self> {
        Some(unsafe { transmute::<&SplMint, &Mint>(SplMint::from_bytes(data)) })
    }

    fn read_mut(_data: &mut [u8]) -> Option<&mut Self> {
        unimplemented!()
    }
}

impl Discriminator for Mint {
    const DISCRIMINATOR: &'static [u8] = &[];
}

impl Owner for Mint {
    const OWNER: Pubkey = ID;
}

#[repr(transparent)]
pub struct Token(SplTokenAccount);

impl RefFromBytes for Token {
    fn read(data: &[u8]) -> Option<&Self> {
        Some(unsafe { transmute::<&SplTokenAccount, &Token>(SplTokenAccount::from_bytes(data)) })
    }

    fn read_mut(_data: &mut [u8]) -> Option<&mut Self> {
        unimplemented!()
    }
}

impl Discriminator for Token {
    const DISCRIMINATOR: &'static [u8] = &[];
}

impl Owner for Token {
    const OWNER: Pubkey = ID;
}

impl Deref for Token {
    type Target = SplTokenAccount;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

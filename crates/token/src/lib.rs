pub use pinocchio_token::instructions::*;
use {
    pinocchio_token::{
        state::{Mint as SplMint, TokenAccount as SplTokenAccount},
        ID,
    },
    std::{mem::transmute, ops::Deref},
    typhoon_program::pubkey::Pubkey,
    typhoon_traits::{Discriminator, Owner, ProgramId, RefFromBytes},
};

pub struct TokenProgram;

impl ProgramId for TokenProgram {
    const ID: Pubkey = pinocchio_token::ID;
}

#[repr(transparent)]
pub struct Mint(SplMint);

impl Mint {
    pub const LEN: usize = SplMint::LEN;
}

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
pub struct TokenAccount(SplTokenAccount);

impl TokenAccount {
    pub const LEN: usize = SplTokenAccount::LEN;
}

impl RefFromBytes for TokenAccount {
    fn read(data: &[u8]) -> Option<&Self> {
        Some(unsafe {
            transmute::<&SplTokenAccount, &TokenAccount>(SplTokenAccount::from_bytes(data))
        })
    }

    fn read_mut(_data: &mut [u8]) -> Option<&mut Self> {
        unimplemented!()
    }
}

impl Discriminator for TokenAccount {
    const DISCRIMINATOR: &'static [u8] = &[];
}

impl Owner for TokenAccount {
    const OWNER: Pubkey = ID;
}

impl Deref for TokenAccount {
    type Target = SplTokenAccount;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

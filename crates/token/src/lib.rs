#![no_std]

use {
    core::{mem::transmute, ops::Deref},
    five8_const::decode_32_const,
    pinocchio::pubkey::{find_program_address, Pubkey},
    pinocchio_associated_token_account::ID as ATA_PROGRAM_ID,
    pinocchio_token::{
        state::{Mint as SplMint, TokenAccount as SplTokenAccount},
        ID as TOKEN_PROGRAM_ID,
    },
    typhoon_accounts::{Discriminator, Owner, Owners, ProgramId, ProgramIds, RefFromBytes},
};

mod traits;

pub use {
    pinocchio_associated_token_account::instructions as ata_instructions,
    pinocchio_token::instructions as spl_instructions, traits::*,
};

const TOKEN_2022_PROGRAM_ID: Pubkey =
    decode_32_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

pub struct AtaTokenProgram;

impl ProgramId for AtaTokenProgram {
    const ID: Pubkey = ATA_PROGRAM_ID;
}

pub struct TokenProgram;

impl ProgramId for TokenProgram {
    const ID: Pubkey = TOKEN_PROGRAM_ID;
}

impl ProgramIds for TokenProgram {
    const IDS: &'static [Pubkey] = &[TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID];
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

    /// Convert a mutable byte slice into a mutable `Mint` reference in zero-copy fashion.
    /// Follows the same constraints as the immutable variant: the caller must guarantee
    /// the slice is at least `Mint::LEN` bytes and correctly aligned.
    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        if data.len() < SplMint::LEN {
            return None;
        }
        Some(unsafe { &mut *(data.as_mut_ptr() as *mut Mint) })
    }
}

impl Discriminator for Mint {
    const DISCRIMINATOR: &'static [u8] = &[];
}

impl Owner for Mint {
    const OWNER: Pubkey = TOKEN_PROGRAM_ID;
}

impl Owners for Mint {
    const OWNERS: &'static [Pubkey] = &[TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID];
}

impl Deref for Mint {
    type Target = SplMint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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

    /// Mutable zero-copy view over a SPL `TokenAccount` byte buffer.
    /// Mirrors the immutable `read` implementation but returns `&mut`.
    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        if data.len() < SplTokenAccount::LEN {
            return None;
        }
        Some(unsafe { &mut *(data.as_mut_ptr() as *mut TokenAccount) })
    }
}

impl Discriminator for TokenAccount {
    const DISCRIMINATOR: &'static [u8] = &[];
}

impl Owner for TokenAccount {
    const OWNER: Pubkey = TOKEN_PROGRAM_ID;
}

impl Owners for TokenAccount {
    const OWNERS: &'static [Pubkey] = &[TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID];
}

impl Deref for TokenAccount {
    type Target = SplTokenAccount;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn find_associated_token_address(mint: &Pubkey, owner: &Pubkey) -> Pubkey {
    find_program_address(
        &[owner.as_ref(), TOKEN_PROGRAM_ID.as_ref(), mint.as_ref()],
        &ATA_PROGRAM_ID,
    )
    .0
}

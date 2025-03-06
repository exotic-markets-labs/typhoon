#[cfg(not(feature = "pinocchio"))]
use {
    nostd_associated_token_account_program::ID as ATA_PROGRAM_ID,
    nostd_token_program::{
        state::{Mint as SplMint, TokenAccount as SplTokenAccount},
        ID as TOKEN_PROGRAM_ID,
    },
};
#[cfg(feature = "pinocchio")]
use {
    pinocchio_associated_token_account::ID as ATA_PROGRAM_ID,
    pinocchio_token::{
        state::{Mint as SplMint, TokenAccount as SplTokenAccount},
        ID as TOKEN_PROGRAM_ID,
    },
};
use {
    std::{mem::transmute, ops::Deref},
    typhoon_accounts::{Discriminator, Owner, ProgramId, RefFromBytes},
    typhoon_program::pubkey::{find_program_address, Pubkey},
};

mod traits;

pub use traits::*;
#[cfg(not(feature = "pinocchio"))]
pub use {
    nostd_associated_token_account_program::instructions as ata_instructions,
    nostd_token_program::instructions as spl_instructions,
};
#[cfg(feature = "pinocchio")]
pub use {
    pinocchio_associated_token_account::instructions as ata_instructions,
    pinocchio_token::instructions as spl_instructions,
};

pub struct AtaTokenProgram;

impl ProgramId for AtaTokenProgram {
    const ID: Pubkey = ATA_PROGRAM_ID;
}

pub struct TokenProgram;

impl ProgramId for TokenProgram {
    const ID: Pubkey = TOKEN_PROGRAM_ID;
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
    const OWNER: Pubkey = TOKEN_PROGRAM_ID;
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

    fn read_mut(_data: &mut [u8]) -> Option<&mut Self> {
        unimplemented!()
    }
}

impl Discriminator for TokenAccount {
    const DISCRIMINATOR: &'static [u8] = &[];
}

impl Owner for TokenAccount {
    const OWNER: Pubkey = TOKEN_PROGRAM_ID;
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

#![no_std]

use {
    core::{marker::PhantomData, mem::transmute, ops::Deref},
    pinocchio::error::ProgramError,
    pinocchio_associated_token_account::ID as ATA_PROGRAM_ID,
    pinocchio_token::{
        state::{Mint as SplMint, TokenAccount as SplTokenAccount},
        ID as TOKEN_PROGRAM_ID,
    },
    solana_address::{address_eq, Address},
    typhoon_traits::{Accessor, AccountStrategy, CheckOwner, CheckProgramId, Discriminator},
};

mod traits;

pub use {
    pinocchio_associated_token_account::instructions as ata_instructions,
    pinocchio_token::instructions as spl_instructions, traits::*,
};

#[cfg(feature = "token2022")]
const TOKEN_2022_PROGRAM_ID: Address =
    Address::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

pub struct AtaTokenProgram;

impl CheckProgramId for AtaTokenProgram {
    #[inline(always)]
    fn address_eq(program_id: &Address) -> bool {
        address_eq(program_id, &ATA_PROGRAM_ID)
    }
}

pub struct TokenProgram;

impl CheckProgramId for TokenProgram {
    #[inline(always)]
    fn address_eq(program_id: &Address) -> bool {
        #[cfg(feature = "token2022")]
        {
            address_eq(program_id, &TOKEN_PROGRAM_ID)
                || address_eq(program_id, &TOKEN_2022_PROGRAM_ID)
        }
        #[cfg(not(feature = "token2022"))]
        {
            address_eq(program_id, &TOKEN_PROGRAM_ID)
        }
    }
}

pub trait SplAccessor {
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of Spl State, and
    /// it is properly aligned to be interpreted as an instance of Spl State.
    /// At the moment Spl State has an alignment of 1 byte.
    /// This method does not perform a length validation.
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self;
}

impl SplAccessor for SplMint {
    #[inline(always)]
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        Self::from_bytes_unchecked(bytes)
    }
}

impl SplAccessor for SplTokenAccount {
    #[inline(always)]
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        Self::from_bytes_unchecked(bytes)
    }
}

pub struct SplStrategy<O: SplAccessor>(PhantomData<O>);

impl<'a, T, O> Accessor<'a, T> for SplStrategy<O>
where
    O: SplAccessor,
    T: 'a,
{
    type Data = &'a T;

    #[inline(always)]
    fn access(data: &'a [u8]) -> Result<Self::Data, ProgramError> {
        Ok(unsafe { transmute::<&O, &T>(O::from_bytes_unchecked(data)) })
    }
}

#[repr(transparent)]
pub struct Mint(SplMint);

impl Mint {
    pub const LEN: usize = SplMint::LEN;
}

impl AccountStrategy for Mint {
    type Strategy = SplStrategy<SplMint>;
}

impl Discriminator for Mint {
    const DISCRIMINATOR: &'static [u8] = &[];
}

impl CheckOwner for Mint {
    #[inline(always)]
    fn owned_by(program_id: &Address) -> bool {
        #[cfg(feature = "token2022")]
        {
            address_eq(program_id, &TOKEN_PROGRAM_ID)
                || address_eq(program_id, &TOKEN_2022_PROGRAM_ID)
        }
        #[cfg(not(feature = "token2022"))]
        {
            address_eq(program_id, &TOKEN_PROGRAM_ID)
        }
    }
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

impl AccountStrategy for TokenAccount {
    type Strategy = SplStrategy<SplTokenAccount>;
}

impl Discriminator for TokenAccount {
    const DISCRIMINATOR: &'static [u8] = &[];
}

impl CheckOwner for TokenAccount {
    #[inline(always)]
    fn owned_by(program_id: &Address) -> bool {
        #[cfg(feature = "token2022")]
        {
            address_eq(program_id, &TOKEN_PROGRAM_ID)
                || address_eq(program_id, &TOKEN_2022_PROGRAM_ID)
        }
        #[cfg(not(feature = "token2022"))]
        {
            address_eq(program_id, &TOKEN_PROGRAM_ID)
        }
    }
}

impl Deref for TokenAccount {
    type Target = SplTokenAccount;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn find_associated_token_address(mint: &Address, owner: &Address) -> Address {
    Address::find_program_address(
        &[owner.as_ref(), TOKEN_PROGRAM_ID.as_ref(), mint.as_ref()],
        &ATA_PROGRAM_ID,
    )
    .0
}

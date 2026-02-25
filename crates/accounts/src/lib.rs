#![no_std]

pub use {accounts::*, discriminator::*, programs::*};
use {
    solana_account_view::{AccountView, Ref, RefMut},
    solana_address::Address,
    solana_program_error::ProgramError,
    typhoon_errors::Error,
    typhoon_traits::Discriminator,
};

mod accounts;
mod discriminator;
mod programs;

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_info(info: &'a AccountView) -> Result<Self, Error>;
}

pub trait ReadableAccount: AsRef<AccountView> {
    #[inline(always)]
    fn address(&self) -> &Address {
        self.as_ref().address()
    }

    #[inline(always)]
    fn owned_by(&self, owner: &Address) -> bool {
        self.as_ref().owned_by(owner)
    }

    #[inline(always)]
    fn lamports(&self) -> u64 {
        self.as_ref().lamports()
    }

    #[inline(always)]
    fn raw_data(&self) -> Result<Ref<'_, [u8]>, ProgramError> {
        self.as_ref().try_borrow()
    }
}

pub trait WritableAccount: ReadableAccount {
    #[inline(always)]
    fn assign(&self, new_owner: &Address) {
        unsafe {
            self.as_ref().assign(new_owner);
        }
    }

    #[inline(always)]
    fn resize(&self, new_len: usize) -> Result<(), Error> {
        self.as_ref().resize(new_len).map_err(Into::into)
    }

    #[inline(always)]
    fn set_lamports(&self, lamports: u64) {
        self.as_ref().set_lamports(lamports);
    }

    #[inline(always)]
    fn raw_mut_data(&self) -> Result<RefMut<'_, [u8]>, ProgramError> {
        self.as_ref().try_borrow_mut()
    }
}

pub trait SignerAccount: ReadableAccount {}

pub trait FromRaw<'a> {
    fn from_raw(info: &'a AccountView) -> Self;
}

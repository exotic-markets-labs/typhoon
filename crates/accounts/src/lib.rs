#![no_std]

pub use {accounts::*, discriminator::*, programs::*};
use {
    bytemuck::{AnyBitPattern, NoUninit},
    pinocchio::{
        account_info::{AccountInfo, Ref, RefMut},
        pubkey::Pubkey,
    },
    typhoon_definitions::Discriminator,
    typhoon_errors::Error,
};

mod accounts;
mod discriminator;
mod programs;

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error>;
}

pub trait ReadableAccount: AsRef<AccountInfo> {
    type DataUnchecked: ?Sized;
    type Data<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.as_ref().key()
    }

    #[inline(always)]
    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.as_ref().is_owned_by(owner)
    }

    #[inline(always)]
    fn lamports(&self) -> Result<Ref<'_, u64>, Error> {
        self.as_ref().try_borrow_lamports().map_err(Into::into)
    }

    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error>;

    fn data_unchecked(&self) -> Result<&Self::DataUnchecked, Error>;
}

pub trait WritableAccount: ReadableAccount {
    type DataMut<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn assign(&self, new_owner: &Pubkey) {
        unsafe {
            self.as_ref().assign(new_owner);
        }
    }

    #[inline(always)]
    fn resize(&self, new_len: usize) -> Result<(), Error> {
        self.as_ref().resize(new_len).map_err(Into::into)
    }

    #[inline(always)]
    fn mut_lamports(&self) -> Result<RefMut<'_, u64>, Error> {
        self.as_ref().try_borrow_mut_lamports().map_err(Into::into)
    }

    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error>;
}

pub trait SignerAccount: ReadableAccount {}

pub trait RefFromBytes {
    fn read(data: &[u8]) -> Option<&Self>;
    fn read_mut(data: &mut [u8]) -> Option<&mut Self>;
}

impl<T> RefFromBytes for T
where
    T: Discriminator + AnyBitPattern + NoUninit,
{
    fn read(data: &[u8]) -> Option<&Self> {
        let dis_len = T::DISCRIMINATOR.len();
        let total_len = dis_len + core::mem::size_of::<T>();

        if data.len() < total_len {
            return None;
        }

        let data_ptr = data[dis_len..total_len].as_ptr();

        if data_ptr.align_offset(core::mem::align_of::<T>()) != 0 {
            return None;
        }

        Some(unsafe { &*(data_ptr as *const T) })
    }

    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        let dis_len = T::DISCRIMINATOR.len();
        let total_len = dis_len + core::mem::size_of::<T>();

        if data.len() < total_len {
            return None;
        }

        let data_ptr = data[dis_len..total_len].as_mut_ptr();

        if data_ptr.align_offset(core::mem::align_of::<T>()) != 0 {
            return None;
        }

        Some(unsafe { &mut *(data_ptr as *mut T) })
    }
}

pub trait FromRaw<'a> {
    fn from_raw(info: &'a AccountInfo) -> Self;
}

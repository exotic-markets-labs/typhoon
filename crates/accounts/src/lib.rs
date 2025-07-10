#![no_std]

pub use {accounts::*, programs::*};
use {
    bytemuck::{AnyBitPattern, NoUninit},
    pinocchio::{
        account_info::{AccountInfo, Ref, RefMut},
        pubkey::Pubkey,
    },
    sealed::Sealed,
    typhoon_errors::Error,
};

mod accounts;
mod programs;

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error>;
}

pub trait ReadableAccount: AsRef<AccountInfo> {
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
}

pub trait WritableAccount: ReadableAccount + Sealed {
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
    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), Error> {
        self.as_ref()
            .realloc(new_len, zero_init)
            .map_err(Into::into)
    }

    #[inline(always)]
    fn mut_lamports(&self) -> Result<RefMut<'_, u64>, Error> {
        self.as_ref().try_borrow_mut_lamports().map_err(Into::into)
    }

    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error>;
}

pub trait SignerAccount: ReadableAccount + Sealed {}

mod sealed {
    use {
        super::{Mut, ReadableAccount, Signer},
        pinocchio::account_info::AccountInfo,
    };

    pub trait Sealed {}

    impl<T> Sealed for Mut<T> where T: ReadableAccount + AsRef<AccountInfo> {}
    impl Sealed for Signer<'_> {}
}

pub trait ProgramId {
    const ID: Pubkey;
}

pub trait ProgramIds {
    const IDS: &'static [Pubkey];
}

pub trait Owner {
    const OWNER: Pubkey;
}

pub trait Owners {
    const OWNERS: &'static [Pubkey];
}

pub trait Discriminator {
    const DISCRIMINATOR: &'static [u8];
}

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

        // Explicit bounds check prevents potential panic in slicing operations.
        // This ensures we have enough bytes for both discriminator and data.
        if data.len() < total_len {
            return None;
        }

        bytemuck::try_from_bytes(&data[dis_len..total_len]).ok()
    }

    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        let dis_len = T::DISCRIMINATOR.len();
        let total_len = dis_len + core::mem::size_of::<T>();

        // Explicit bounds check prevents potential panic in slicing operations.
        // This ensures we have enough bytes for both discriminator and data.
        if data.len() < total_len {
            return None;
        }

        bytemuck::try_from_bytes_mut(&mut data[dis_len..total_len]).ok()
    }
}

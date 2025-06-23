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

    fn key(&self) -> &Pubkey;
    fn is_owned_by(&self, owner: &Pubkey) -> bool;
    fn lamports(&self) -> Result<Ref<'_, u64>, Error>;
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error>;
}

pub trait WritableAccount: ReadableAccount + Sealed {
    type DataMut<'a>
    where
        Self: 'a;

    fn assign(&self, new_owner: &Pubkey);
    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), Error>;
    fn mut_lamports(&self) -> Result<RefMut<'_, u64>, Error>;
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error>;
}

pub trait SignerAccount: ReadableAccount + Sealed {}

/// Trait for batch validation of multiple accounts of the same type.
/// 
/// This trait enables optimized validation patterns when processing many accounts,
/// reducing syscall overhead through batching and early filtering.
pub trait BatchValidation {
    /// Validate multiple accounts of the same type in a single pass.
    /// Implementations should use early-exit strategies and minimize syscalls.
    fn validate_batch(accounts: &[&AccountInfo]) -> Result<(), Error>;
    
    /// Fast pre-validation check without full deserialization.
    /// Used for early filtering before expensive validation operations.
    fn pre_validate(info: &AccountInfo) -> bool;
}

/// Cache for AccountInfo data to avoid repeated borrows during validation.
/// 
/// This cache stores borrowed data and lamports for the lifetime of the cache,
/// reducing syscall overhead when multiple validation checks need the same data.
pub struct AccountInfoCache<'a> {
    info: &'a AccountInfo,
    cached_data: Option<&'a [u8]>,
    cached_lamports: Option<u64>,
}

impl<'a> AccountInfoCache<'a> {
    #[inline(always)]
    pub fn new(info: &'a AccountInfo) -> Self {
        Self {
            info,
            cached_data: None,
            cached_lamports: None,
        }
    }

    /// Get account data with caching to avoid repeated borrows.
    /// 
    /// The cached reference is valid for the lifetime of the cache because
    /// AccountInfo borrows are checked at runtime.
    #[inline(always)]
    pub fn data(&mut self) -> Result<&'a [u8], Error> {
        if self.cached_data.is_none() {
            let data_ref = self.info.try_borrow_data()?;
            self.cached_data = Some(unsafe { 
                core::slice::from_raw_parts(
                    data_ref.as_ptr(), 
                    data_ref.len()
                ) 
            });
        }
        Ok(self.cached_data.unwrap())
    }

    /// Get lamports with caching to avoid repeated borrows.
    #[inline(always)]
    pub fn lamports(&mut self) -> Result<u64, Error> {
        if self.cached_lamports.is_none() {
            self.cached_lamports = Some(*self.info.try_borrow_lamports()?);
        }
        Ok(self.cached_lamports.unwrap())
    }
}

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
        
        // Check bounds before slicing
        if data.len() < total_len {
            return None;
        }
        
        bytemuck::try_from_bytes(&data[dis_len..total_len]).ok()
    }

    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        let dis_len = T::DISCRIMINATOR.len();
        let total_len = dis_len + core::mem::size_of::<T>();
        
        // Check bounds before slicing
        if data.len() < total_len {
            return None;
        }
        
        bytemuck::try_from_bytes_mut(&mut data[dis_len..total_len]).ok()
    }
}

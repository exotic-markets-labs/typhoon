pub use {accounts::*, programs::*};
use {
    bytemuck::Pod,
    pinocchio::{
        account_info::{AccountInfo, Ref, RefMut},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    sealed::Sealed,
};

mod accounts;
mod programs;

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, ProgramError>;
}

pub trait ReadableAccount: AsRef<AccountInfo> {
    type DataType: ?Sized;

    fn key(&self) -> &Pubkey;
    fn is_owned_by(&self, owner: &Pubkey) -> bool;
    fn lamports(&self) -> Result<Ref<u64>, ProgramError>;
    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError>;
}

pub trait WritableAccount: ReadableAccount + Sealed {
    fn assign(&self, new_owner: &Pubkey);
    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), ProgramError>;
    fn mut_lamports(&self) -> Result<RefMut<u64>, ProgramError>;
    fn mut_data(&self) -> Result<RefMut<Self::DataType>, ProgramError>;
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

pub trait Owner {
    const OWNER: Pubkey;
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
    T: Pod + Discriminator,
{
    fn read(data: &[u8]) -> Option<&Self> {
        let dis_len = T::DISCRIMINATOR.len();
        bytemuck::try_from_bytes(&data[dis_len..std::mem::size_of::<T>() + dis_len]).ok()
    }

    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        let dis_len = T::DISCRIMINATOR.len();
        bytemuck::try_from_bytes_mut(&mut data[dis_len..std::mem::size_of::<T>() + dis_len]).ok()
    }
}

mod accounts;
mod programs;

pub use {accounts::*, programs::*};
use {
    bytemuck::Pod,
    sealed::Sealed,
    typhoon_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref, RefMut},
};

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError>;
}

pub trait ReadableAccount: AsRef<RawAccountInfo> {
    type DataType: ?Sized;

    fn key(&self) -> &Pubkey;
    fn owner(&self) -> &Pubkey;
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
        typhoon_program::RawAccountInfo,
    };

    pub trait Sealed {}

    impl<T> Sealed for Mut<T> where T: ReadableAccount + AsRef<RawAccountInfo> {}
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

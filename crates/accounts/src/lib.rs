mod accounts;
mod programs;

pub use {accounts::*, programs::*};
use {
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

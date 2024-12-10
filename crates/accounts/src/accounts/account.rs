use {
    crate::{FromAccountInfo, Owner, ReadableAccount},
    aligned::{Aligned, A8},
    bytemuck::Pod,
    std::marker::PhantomData,
    typhoon_errors::Error,
    typhoon_program::{
        bytes::try_from_bytes, program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref,
    },
};

pub struct Account<'a, T>
where
    T: Pod,
{
    info: &'a RawAccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: Owner + Pod,
{
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.owner() != &T::OWNER {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        Ok(Account {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<T> AsRef<RawAccountInfo> for Account<'_, T>
where
    T: Pod,
{
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Account<'_, T>
where
    T: Pod,
{
    type DataType = T;

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn owner(&self) -> &Pubkey {
        self.info.owner()
    }

    fn lamports(&self) -> Result<Ref<u64>, ProgramError> {
        self.info.try_borrow_lamports()
    }

    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError> {
        let data = self.info.try_borrow_data()?;

        Ref::filter_map(data, |data| {
            try_from_bytes(&data[..std::mem::size_of::<Aligned<A8, Self>>()])
        })
        .map_err(|_| ProgramError::InvalidAccountData)
    }
}

use {
    crate::{Discriminator, FromAccountInfo, Owner, ReadableAccount},
    bytemuck::Pod,
    std::marker::PhantomData,
    typhoon_errors::Error,
    typhoon_program::{
        bytes::try_from_bytes, program_error::ProgramError, pubkey::Pubkey, system_program,
        RawAccountInfo, Ref,
    },
};

pub struct Account<'a, T>
where
    T: Pod + Discriminator,
{
    info: &'a RawAccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: Owner + Pod + Discriminator,
{
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.owner() == &system_program::ID && *info.try_borrow_lamports()? == 0 {
            return Err(ProgramError::UninitializedAccount);
        }

        if info.owner() != &T::OWNER {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        let account_data = info.try_borrow_data()?;

        if account_data.len() < T::DISCRIMINATOR.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        if T::DISCRIMINATOR != &account_data[..T::DISCRIMINATOR.len()] {
            return Err(Error::AccountDiscriminatorMismatch.into());
        }

        Ok(Account {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<T> AsRef<RawAccountInfo> for Account<'_, T>
where
    T: Pod + Discriminator,
{
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Account<'_, T>
where
    T: Pod + Discriminator,
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
        let dis_len = T::DISCRIMINATOR.len();
        let data = self.info.try_borrow_data()?;

        Ref::filter_map(data, |data| {
            try_from_bytes(&data[dis_len..std::mem::size_of::<T>() + dis_len])
        })
        .map_err(|_| ProgramError::InvalidAccountData)
    }
}

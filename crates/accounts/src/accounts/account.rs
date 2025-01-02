use {
    crate::{Discriminator, FromAccountInfo, Owner, ReadableAccount},
    std::marker::PhantomData,
    typhoon_errors::Error,
    typhoon_program::{
        program_error::ProgramError, pubkey::Pubkey, system_program, RawAccountInfo, Ref,
    },
    zerocopy::{FromBytes, Immutable, KnownLayout},
};

pub struct Account<'a, T>
where
    T: Discriminator,
{
    info: &'a RawAccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: Owner + Discriminator,
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
    T: Discriminator,
{
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Account<'_, T>
where
    T: FromBytes + KnownLayout + Immutable + Discriminator,
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
            let (dis, state) = T::ref_from_suffix(data).ok()?;

            if T::DISCRIMINATOR.len() != dis.len() {
                return None;
            }

            Some(state)
        })
        .map_err(|_| ProgramError::InvalidAccountData)
    }
}

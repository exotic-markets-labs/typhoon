use {
    crate::{
        utils::fast_32_byte_eq, Discriminator, FromAccountInfo, FromRaw, Owner, ReadableAccount,
        RefFromBytes,
    },
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct Account<'a, T>
where
    T: Discriminator + RefFromBytes,
{
    pub(crate) info: &'a AccountInfo,
    pub(crate) _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: Owner + Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if fast_32_byte_eq(info.owner(), &pinocchio_system::ID) && *info.try_borrow_lamports()? == 0
        {
            return Err(ProgramError::UninitializedAccount.into());
        }

        if !fast_32_byte_eq(info.owner(), &T::OWNER) {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        let account_data = info.try_borrow_data()?;

        if account_data.len() < T::DISCRIMINATOR.len() {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        if T::DISCRIMINATOR != &account_data[..T::DISCRIMINATOR.len()] {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }

        Ok(Account {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Account<'a, T>> for &'a AccountInfo
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn from(value: Account<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for Account<'_, T>
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Account<'_, T>
where
    T: RefFromBytes + Discriminator,
{
    type Data<'a>
        = Ref<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ref::filter_map(self.info.try_borrow_data()?, T::read)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<'a, T> FromRaw<'a> for Account<'a, T>
where
    T: RefFromBytes + Discriminator,
{
    fn from_raw(info: &'a AccountInfo) -> Self {
        Self {
            info,
            _phantom: PhantomData,
        }
    }
}

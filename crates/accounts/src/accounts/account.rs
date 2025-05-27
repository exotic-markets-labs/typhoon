use {
    crate::{Discriminator, FromAccountInfo, Owner, ReadableAccount, RefFromBytes},
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct Account<'a, T>
where
    T: Discriminator + RefFromBytes,
{
    info: &'a AccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: Owner + Discriminator + RefFromBytes,
{
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if info.is_owned_by(&pinocchio_system::ID) && *info.try_borrow_lamports()? == 0 {
            return Err(ProgramError::UninitializedAccount.into());
        }

        if !info.is_owned_by(&T::OWNER) {
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
    T: Owner + Discriminator + RefFromBytes,
{
    fn from(value: Account<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for Account<'_, T>
where
    T: Discriminator + RefFromBytes,
{
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

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.info.is_owned_by(owner)
    }

    fn lamports(&self) -> Result<Ref<u64>, Error> {
        self.info.try_borrow_lamports().map_err(Into::into)
    }

    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ref::filter_map(self.info.try_borrow_data()?, T::read)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

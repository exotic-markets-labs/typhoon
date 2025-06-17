use {
    crate::{Discriminator, FromAccountInfo, Owners, ReadableAccount, RefFromBytes},
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct InterfaceAccount<'a, T>
where
    T: Discriminator + RefFromBytes,
{
    info: &'a AccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for InterfaceAccount<'a, T>
where
    T: Discriminator + RefFromBytes + Owners,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if info.is_owned_by(&pinocchio_system::ID) && *info.try_borrow_lamports()? == 0 {
            return Err(ProgramError::UninitializedAccount.into());
        }

        // Safe because we don't store the owner key
        if !T::OWNERS.contains(unsafe { info.owner() }) {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        let account_data = info.try_borrow_data()?;

        if account_data.len() < T::DISCRIMINATOR.len() {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        if T::DISCRIMINATOR != &account_data[..T::DISCRIMINATOR.len()] {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }

        Ok(InterfaceAccount {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<InterfaceAccount<'a, T>> for &'a AccountInfo
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn from(value: InterfaceAccount<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for InterfaceAccount<'_, T>
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for InterfaceAccount<'_, T>
where
    T: RefFromBytes + Discriminator,
{
    type Data<'a>
        = Ref<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    #[inline(always)]
    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.info.is_owned_by(owner)
    }

    #[inline(always)]
    fn lamports(&self) -> Result<Ref<'_, u64>, Error> {
        self.info.try_borrow_lamports().map_err(Into::into)
    }

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ref::filter_map(self.info.try_borrow_data()?, T::read)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

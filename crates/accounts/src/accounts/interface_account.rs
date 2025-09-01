use {
    crate::{
        discriminator_matches, internal::unlikely, Discriminator, FromAccountInfo, FromRaw, Owners,
        ReadableAccount, RefFromBytes,
    },
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct InterfaceAccount<'a, T>
where
    T: Discriminator + RefFromBytes,
{
    pub(crate) info: &'a AccountInfo,
    pub(crate) _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for InterfaceAccount<'a, T>
where
    T: Discriminator + RefFromBytes + Owners,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        let account_data = info.try_borrow_data()?;

        // Check data length first - this is the cheapest check and most likely to fail
        if unlikely(account_data.len() < T::DISCRIMINATOR.len()) {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        // Validate discriminator using optimized comparison for small discriminators
        if unlikely(!discriminator_matches::<T>(&account_data)) {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }

        // Verify account ownership against multiple allowed owners - checked after discriminator for better branch prediction
        if unlikely(!T::OWNERS.contains(info.owner())) {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        // Handle special case: zero-lamport system accounts (least common case)
        if unlikely(info.is_owned_by(&pinocchio_system::ID)) {
            // Only perform additional lamports check for system accounts
            if *info.try_borrow_lamports()? == 0 {
                return Err(ProgramError::UninitializedAccount.into());
            }
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
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ref::filter_map(self.info.try_borrow_data()?, T::read)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<'a, T> FromRaw<'a> for InterfaceAccount<'a, T>
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

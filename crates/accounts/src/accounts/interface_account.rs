use {
    crate::{
        discriminator_matches, AccountData, Discriminator, FromAccountInfo, FromRaw,
        ReadableAccount, System,
    },
    core::marker::PhantomData,
    pinocchio::hint::unlikely,
    solana_account_view::AccountView,
    solana_program_error::ProgramError,
    typhoon_errors::{Error, ErrorCode},
    typhoon_traits::{Owners, ProgramId},
};

pub struct InterfaceAccount<'a, T>
where
    T: Discriminator,
{
    pub(crate) info: &'a AccountView,
    pub(crate) _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for InterfaceAccount<'a, T>
where
    T: Discriminator + Owners,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountView) -> Result<Self, Error> {
        // Check data length first - this is the cheapest check and most likely to fail
        if unlikely(info.data_len() < T::DISCRIMINATOR.len()) {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        // Validate discriminator using optimized comparison for small discriminators
        if unlikely(!discriminator_matches::<T>(info)) {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }

        // Verify account ownership against multiple allowed owners - checked after discriminator for better branch prediction
        if unlikely(!T::OWNERS.contains(unsafe { info.owner() })) {
            return Err(ProgramError::InvalidAccountOwner.into());
        }

        // Handle special case: zero-lamport system accounts (least common case)
        if unlikely(info.owned_by(&System::ID)) {
            // Only perform additional lamports check for system accounts
            if info.lamports() == 0 {
                return Err(ProgramError::UninitializedAccount.into());
            }
        }

        Ok(InterfaceAccount {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<InterfaceAccount<'a, T>> for &'a AccountView
where
    T: Discriminator,
{
    #[inline(always)]
    fn from(value: InterfaceAccount<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountView> for InterfaceAccount<'_, T>
where
    T: Discriminator,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountView {
        self.info
    }
}

impl<T> ReadableAccount for InterfaceAccount<'_, T> where T: Discriminator {}

impl<T> AccountData for InterfaceAccount<'_, T>
where
    T: Discriminator + typhoon_traits::AccountStrategy,
{
    type Data = T;
}

impl<'a, T> FromRaw<'a> for InterfaceAccount<'a, T>
where
    T: Discriminator,
{
    fn from_raw(info: &'a AccountView) -> Self {
        Self {
            info,
            _phantom: PhantomData,
        }
    }
}

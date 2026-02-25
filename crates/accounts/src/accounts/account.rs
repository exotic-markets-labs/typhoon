use {
    crate::{discriminator_matches, FromAccountInfo, FromRaw, ReadableAccount, System},
    core::marker::PhantomData,
    pinocchio::hint::unlikely,
    solana_account_view::{AccountView, Ref},
    solana_address::address_eq,
    solana_program_error::ProgramError,
    typhoon_errors::{Error, ErrorCode},
    typhoon_traits::{Accessor, AccountStrategy, Discriminator, Owner, ProgramId},
};

pub struct Account<'a, T>
where
    T: Discriminator,
{
    pub(crate) info: &'a AccountView,
    pub(crate) _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: Owner + Discriminator,
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

        let owner = unsafe { info.owner() };

        // Verify account ownership - checked after discriminator for better branch prediction
        if unlikely(!address_eq(owner, &T::OWNER)) {
            return Err(ProgramError::InvalidAccountOwner.into());
        }

        // Handle special case: zero-lamport system accounts (least common case)
        if unlikely(address_eq(owner, &System::ID)) {
            // Only perform additional lamports check for system accounts
            if info.lamports() == 0 {
                return Err(ProgramError::UninitializedAccount.into());
            }
        }

        Ok(Account {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Account<'a, T>> for &'a AccountView
where
    T: Discriminator,
{
    #[inline(always)]
    fn from(value: Account<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountView> for Account<'_, T>
where
    T: Discriminator,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountView {
        self.info
    }
}

impl<T> Account<'_, T>
where
    T: Discriminator + AccountStrategy,
{
    #[inline(always)]
    pub fn data(&self) -> Result<Ref<'_, T>, ProgramError>
    where
        <T as AccountStrategy>::Strategy: for<'a> Accessor<'a, T, Data = &'a T>,
    {
        Ref::try_map(self.info.try_borrow()?, |data| {
            <<T as AccountStrategy>::Strategy as Accessor<'_, T>>::access(
                &data[T::DISCRIMINATOR.len()..],
            )
        })
        .map_err(|_| ProgramError::InvalidAccountData)
    }

    #[inline(always)]
    pub fn data_owned(&self) -> Result<T, ProgramError>
    where
        <T as AccountStrategy>::Strategy: for<'a> Accessor<'a, T, Data = T>,
    {
        self.info.check_borrow()?;
        let data = unsafe { self.info.borrow_unchecked() };
        <<T as AccountStrategy>::Strategy as Accessor<'_, T>>::access(
            &data[T::DISCRIMINATOR.len()..],
        )
    }

    #[inline(always)]
    pub fn data_unchecked(
        &self,
    ) -> Result<<<T as AccountStrategy>::Strategy as Accessor<'_, T>>::Data, ProgramError>
    where
        <T as AccountStrategy>::Strategy: for<'a> Accessor<'a, T, Data = T>,
    {
        let data = unsafe { self.info.borrow_unchecked() };
        <<T as AccountStrategy>::Strategy as Accessor<'_, T>>::access(
            &data[T::DISCRIMINATOR.len()..],
        )
    }
}

impl<T> ReadableAccount for Account<'_, T> where T: Discriminator {}

impl<'a, T> FromRaw<'a> for Account<'a, T>
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

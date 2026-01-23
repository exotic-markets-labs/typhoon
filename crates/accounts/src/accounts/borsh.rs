use {
    super::Mut,
    crate::{
        discriminator_matches, Discriminator, FromAccountInfo, ReadableAccount, Signer, System,
        WritableAccount,
    },
    core::cell::RefCell,
    pinocchio::hint::unlikely,
    solana_account_view::AccountView,
    solana_address::address_eq,
    solana_program_error::ProgramError,
    typhoon_errors::{Error, ErrorCode},
    typhoon_traits::{Owner, ProgramId},
};

pub struct BorshAccount<'a, T>
where
    T: Discriminator,
{
    info: &'a AccountView,
    data: RefCell<T>,
}

impl<'a, T> FromAccountInfo<'a> for BorshAccount<'a, T>
where
    T: Owner + Discriminator + borsh::BorshSerialize + borsh::BorshDeserialize,
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

        // Deserialize the state data (this is the most expensive operation, done last)
        let state =
            T::deserialize(&mut &unsafe { info.borrow_unchecked() }[T::DISCRIMINATOR.len()..])
                .map_err(|_| ProgramError::BorshIoError)?;

        Ok(BorshAccount {
            info,
            data: RefCell::new(state),
        })
    }
}

impl<'a, T> From<BorshAccount<'a, T>> for &'a AccountView
where
    T: Owner + Discriminator,
{
    #[inline(always)]
    fn from(value: BorshAccount<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountView> for BorshAccount<'_, T>
where
    T: Discriminator,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountView {
        self.info
    }
}

impl<T> ReadableAccount for BorshAccount<'_, T>
where
    T: Discriminator,
{
    type DataUnchecked = T;
    type Data<'a>
        = core::cell::Ref<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ok(self.data.borrow())
    }

    #[inline]
    fn data_unchecked(&self) -> Result<&Self::DataUnchecked, Error> {
        Ok(unsafe {
            self.data
                .try_borrow_unguarded()
                .map_err(|_| ProgramError::AccountBorrowFailed)?
        })
    }
}

impl<T> WritableAccount for Mut<BorshAccount<'_, T>>
where
    T: Discriminator,
{
    type DataMut<'a>
        = core::cell::RefMut<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        self.0
            .data
            .try_borrow_mut()
            .map_err(|_| ProgramError::AccountBorrowFailed.into())
    }
}

impl<T> WritableAccount for Mut<Signer<'_, BorshAccount<'_, T>>>
where
    T: Discriminator,
{
    type DataMut<'a>
        = core::cell::RefMut<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        self.0
            .acc
            .data
            .try_borrow_mut()
            .map_err(|_| ProgramError::AccountBorrowFailed.into())
    }
}

impl<T> Mut<BorshAccount<'_, T>>
where
    T: Discriminator + borsh::BorshSerialize,
{
    #[inline(always)]
    pub fn serialize(&self) -> Result<(), Error> {
        let data = self
            .0
            .data
            .try_borrow()
            .map_err(|_| ProgramError::AccountBorrowFailed)?;

        data.serialize(&mut self.0.info.try_borrow_mut()?.as_mut())
            .map_err(|_| ProgramError::BorshIoError.into())
    }
}

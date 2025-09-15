use {
    super::Mut,
    crate::{
        discriminator_matches, Discriminator, FromAccountInfo, Owner, ReadableAccount,
        WritableAccount,
    },
    core::cell::RefCell,
    pinocchio::{
        account_info::AccountInfo, hint::unlikely, program_error::ProgramError, pubkey::pubkey_eq,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct BorshAccount<'a, T>
where
    T: Discriminator,
{
    info: &'a AccountInfo,
    data: RefCell<T>,
}

impl<'a, T> FromAccountInfo<'a> for BorshAccount<'a, T>
where
    T: Owner + Discriminator + borsh::BorshSerialize + borsh::BorshDeserialize,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        // Borrow account data once for all validation checks and deserialization
        let account_data = info.try_borrow_data()?;

        // Check data length first - this is the cheapest check and most likely to fail
        if unlikely(account_data.len() < T::DISCRIMINATOR.len()) {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        // Split data once for validation and deserialization
        let (discriminator, mut data) = account_data.split_at(T::DISCRIMINATOR.len());

        // Validate discriminator using optimized comparison for small discriminators
        if unlikely(!discriminator_matches::<T>(discriminator)) {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }

        // Verify account ownership - checked after discriminator for better branch prediction
        if unlikely(!pubkey_eq(info.owner(), &T::OWNER)) {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        // Handle special case: zero-lamport system accounts (least common case)
        if unlikely(pubkey_eq(info.owner(), &pinocchio_system::ID)) {
            // Only perform additional lamports check for system accounts
            if *info.try_borrow_lamports()? == 0 {
                return Err(ProgramError::UninitializedAccount.into());
            }
        }

        // Deserialize the state data (this is the most expensive operation, done last)
        let state = T::deserialize(&mut data).map_err(|_| ProgramError::BorshIoError)?;

        Ok(BorshAccount {
            info,
            data: RefCell::new(state),
        })
    }
}

impl<'a, T> From<BorshAccount<'a, T>> for &'a AccountInfo
where
    T: Owner + Discriminator,
{
    #[inline(always)]
    fn from(value: BorshAccount<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for BorshAccount<'_, T>
where
    T: Discriminator,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for BorshAccount<'_, T>
where
    T: Discriminator,
{
    type Data<'a>
        = core::cell::Ref<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ok(self.data.borrow())
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

        data.serialize(&mut self.0.info.try_borrow_mut_data()?.as_mut())
            .map_err(|_| ProgramError::BorshIoError.into())
    }
}

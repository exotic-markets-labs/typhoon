use {
    super::Mut,
    crate::{
        utils::fast_32_byte_eq, Discriminator, FromAccountInfo, Owner, ReadableAccount,
        WritableAccount,
    },
    core::cell::RefCell,
    pinocchio::{account_info::AccountInfo, program_error::ProgramError},
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

        let (discriminator, mut data) = account_data.split_at(T::DISCRIMINATOR.len());

        if T::DISCRIMINATOR != discriminator {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }

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

use {
    super::Mut,
    crate::{Discriminator, FromAccountInfo, Owner, ReadableAccount, WritableAccount},
    core::cell::RefCell,
    pinocchio::{
        account_info::{AccountInfo, Ref, RefMut},
        program_error::ProgramError,
        pubkey::Pubkey,
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
    fn from(value: BorshAccount<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for BorshAccount<'_, T>
where
    T: Discriminator,
{
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

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.info.is_owned_by(owner)
    }

    fn lamports(&self) -> Result<Ref<'_, u64>, Error> {
        self.info.try_borrow_lamports().map_err(Into::into)
    }

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

    fn assign(&self, new_owner: &Pubkey) {
        unsafe {
            self.0.as_ref().assign(new_owner);
        }
    }

    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), Error> {
        self.0
            .as_ref()
            .realloc(new_len, zero_init)
            .map_err(Into::into)
    }

    fn mut_lamports(&self) -> Result<RefMut<'_, u64>, Error> {
        self.0
            .as_ref()
            .try_borrow_mut_lamports()
            .map_err(Into::into)
    }

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

use {
    crate::{Discriminator, FromAccountInfo, Owner, ReadableAccount},
    typhoon_errors::Error,
    typhoon_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref},
};

pub struct BorshAccount<'a, T>
where
    T: Discriminator + borsh::BorshSerialize + borsh::BorshDeserialize,
{
    info: &'a RawAccountInfo,
    data: T,
}

impl<T> BorshAccount<'_, T>
where
    T: Owner + Discriminator + borsh::BorshSerialize + borsh::BorshDeserialize,
{
    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<'a, T> FromAccountInfo<'a> for BorshAccount<'a, T>
where
    T: Owner + Discriminator + borsh::BorshSerialize + borsh::BorshDeserialize,
{
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.owner() != &T::OWNER {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        let account_data = info.try_borrow_data()?;
        let (discriminator, mut data) = account_data.split_at(T::DISCRIMINATOR.len());

        if T::DISCRIMINATOR != discriminator {
            return Err(Error::AccountDiscriminatorMismatch.into());
        }

        let state = T::deserialize(&mut data).map_err(|_| Error::BorshIoError)?;

        Ok(BorshAccount { info, data: state })
    }
}

impl<T> AsRef<RawAccountInfo> for BorshAccount<'_, T>
where
    T: borsh::BorshSerialize + borsh::BorshDeserialize + Discriminator,
{
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for BorshAccount<'_, T>
where
    T: borsh::BorshSerialize + borsh::BorshDeserialize + Discriminator,
{
    type DataType = [u8];

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn owner(&self) -> &Pubkey {
        self.info.owner()
    }

    fn lamports(&self) -> Result<Ref<u64>, ProgramError> {
        self.info.try_borrow_lamports()
    }

    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError> {
        self.info.try_borrow_data()
    }
}

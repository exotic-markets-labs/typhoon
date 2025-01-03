use {
    crate::{FromAccountInfo, ReadMut, ReadableAccount, Signer, SignerAccount, WritableAccount},
    typhoon_errors::Error,
    typhoon_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref, RefMut},
};

pub struct Mut<T: ReadableAccount>(T);

impl<'a, T> FromAccountInfo<'a> for Mut<T>
where
    T: FromAccountInfo<'a> + ReadableAccount,
{
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if !info.is_writable() {
            return Err(Error::AccountNotMutable.into());
        }

        Ok(Mut(T::try_from_info(info)?))
    }
}

impl<T> AsRef<RawAccountInfo> for Mut<T>
where
    T: ReadableAccount,
{
    fn as_ref(&self) -> &RawAccountInfo {
        self.0.as_ref()
    }
}

impl<T> ReadableAccount for Mut<T>
where
    T: ReadableAccount,
{
    type DataType = T::DataType;

    fn key(&self) -> &Pubkey {
        self.0.key()
    }

    fn owner(&self) -> &Pubkey {
        self.0.owner()
    }

    fn lamports(&self) -> Result<Ref<u64>, ProgramError> {
        self.0.lamports()
    }

    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError> {
        self.0.data()
    }
}

impl<T, U> WritableAccount for Mut<T>
where
    T: ReadableAccount<DataType = U>,
    U: ReadMut + ?Sized,
{
    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), ProgramError> {
        self.0.as_ref().realloc(new_len, zero_init)
    }

    fn mut_lamports(&self) -> Result<RefMut<u64>, ProgramError> {
        self.0.as_ref().try_borrow_mut_lamports()
    }

    fn mut_data(&self) -> Result<RefMut<Self::DataType>, ProgramError> {
        let data = self.0.as_ref().try_borrow_mut_data()?;

        RefMut::filter_map(data, T::DataType::read_mut)
            .map_err(|_| ProgramError::InvalidAccountData)
    }
}

impl SignerAccount for Mut<Signer<'_>> {}

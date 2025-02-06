use {
    crate::{FromAccountInfo, ReadableAccount},
    std::marker::PhantomData,
    typhoon_errors::Error,
    typhoon_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref},
    typhoon_traits::ProgramId,
};

///
/// Checks:
/// * `account_info.key == expected_program`
/// * `account_info.executable == true`
pub struct Program<'a, T> {
    info: &'a RawAccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Program<'a, T>
where
    T: ProgramId,
{
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.key() != &T::ID {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        if !info.executable() {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        Ok(Program {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Program<'a, T>> for &'a RawAccountInfo {
    fn from(value: Program<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<RawAccountInfo> for Program<'_, T> {
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Program<'_, T> {
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

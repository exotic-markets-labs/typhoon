use {
    crate::{FromAccountInfo, ProgramId, ReadableAccount},
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    std::marker::PhantomData,
    typhoon_errors::Error,
};

///
/// Checks:
/// * `account_info.key == expected_program`
/// * `account_info.executable == true`
pub struct Program<'a, T> {
    info: &'a AccountInfo,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Program<'a, T>
where
    T: ProgramId,
{
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, ProgramError> {
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

impl<'a, T> From<Program<'a, T>> for &'a AccountInfo {
    fn from(value: Program<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for Program<'_, T> {
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Program<'_, T> {
    type DataType = [u8];

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.info.is_owned_by(owner)
    }

    fn lamports(&self) -> Result<Ref<u64>, ProgramError> {
        self.info.try_borrow_lamports()
    }

    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError> {
        self.info.try_borrow_data()
    }
}

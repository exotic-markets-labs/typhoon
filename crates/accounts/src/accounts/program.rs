use {
    crate::{FromAccountInfo, ProgramId, ReadableAccount},
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    typhoon_errors::{Error, ErrorCode},
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
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        // program ID check with compile-time constant
        Self::validate_program_id(info)?;

        if !info.executable() {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        Ok(Program {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> Program<'a, T>
where
    T: ProgramId,
{
    /// program ID validation using compile-time constants
    /// This function is inlined and the compiler can optimize the check since T::ID is known at compile time
    #[inline(always)]
    fn validate_program_id(info: &AccountInfo) -> Result<(), Error> {
        // The compiler can optimize this check since T::ID is a compile-time constant
        if info.key() != &T::ID {
            return Err(ProgramError::IncorrectProgramId.into());
        }
        Ok(())
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
    type Data<'a>
        = Ref<'a, [u8]>
    where
        Self: 'a;

    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    #[inline(always)]
    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.info.is_owned_by(owner)
    }

    fn lamports(&self) -> Result<Ref<'_, u64>, Error> {
        self.info.try_borrow_lamports().map_err(Into::into)
    }

    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.info.try_borrow_data().map_err(Into::into)
    }
}

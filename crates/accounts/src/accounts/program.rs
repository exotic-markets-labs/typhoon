use {
    crate::{FromAccountInfo, ProgramId, ReadableAccount},
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        hint::unlikely,
        program_error::ProgramError,
        pubkey::pubkey_eq,
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
    #[inline]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        // Optimized program ID check using fast memory comparison
        if unlikely(!pubkey_eq(info.key(), &T::ID)) {
            return Err(ProgramError::IncorrectProgramId.into());
        }

        if !info.executable() {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        Ok(Program {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Program<'a, T>> for &'a AccountInfo {
    #[inline(always)]
    fn from(value: Program<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for Program<'_, T> {
    #[inline(always)]
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
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.info.try_borrow_data().map_err(Into::into)
    }
}

use {
    crate::{FromAccountInfo, ReadableAccount},
    core::marker::PhantomData,
    pinocchio::hint::unlikely,
    solana_account_view::AccountView,
    solana_address::address_eq,
    solana_program_error::ProgramError,
    typhoon_errors::Error,
    typhoon_traits::ProgramId,
};

///
/// Checks:
/// * `account_info.key == expected_program`
/// * `account_info.executable == true`
pub struct Program<'a, T> {
    info: &'a AccountView,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Program<'a, T>
where
    T: ProgramId,
{
    #[inline]
    fn try_from_info(info: &'a AccountView) -> Result<Self, Error> {
        // Optimized program ID check using fast memory comparison
        if unlikely(!address_eq(info.address(), &T::ID)) {
            return Err(ProgramError::IncorrectProgramId.into());
        }

        if !info.executable() {
            return Err(ProgramError::InvalidAccountOwner.into());
        }

        Ok(Program {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Program<'a, T>> for &'a AccountView {
    #[inline(always)]
    fn from(value: Program<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountView> for Program<'_, T> {
    #[inline(always)]
    fn as_ref(&self) -> &AccountView {
        self.info
    }
}

impl<T> ReadableAccount for Program<'_, T> {}

use {
    crate::{FromAccountInfo, ReadableAccount},
    core::marker::PhantomData,
    solana_account_view::AccountView,
    solana_program_error::ProgramError,
    typhoon_errors::Error,
    typhoon_traits::ProgramIds,
};

pub struct Interface<'a, T> {
    info: &'a AccountView,
    _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Interface<'a, T>
where
    T: ProgramIds,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountView) -> Result<Self, Error> {
        if !T::IDS.contains(info.address()) {
            return Err(ProgramError::IncorrectProgramId.into());
        }

        if !info.executable() {
            return Err(ProgramError::InvalidAccountOwner.into());
        }

        Ok(Interface {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Interface<'a, T>> for &'a AccountView {
    #[inline(always)]
    fn from(value: Interface<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountView> for Interface<'_, T> {
    #[inline(always)]
    fn as_ref(&self) -> &AccountView {
        self.info
    }
}

impl<T> ReadableAccount for Interface<'_, T> {}

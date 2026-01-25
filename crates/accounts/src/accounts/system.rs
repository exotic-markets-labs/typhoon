use {
    crate::{FromAccountInfo, ReadableAccount, System},
    pinocchio::hint::unlikely,
    solana_account_view::{AccountView, Ref},
    solana_address::address_eq,
    solana_program_error::ProgramError,
    typhoon_errors::Error,
    typhoon_traits::ProgramId,
};

pub struct SystemAccount<'a> {
    info: &'a AccountView,
}

impl<'a> FromAccountInfo<'a> for SystemAccount<'a> {
    #[inline(always)]
    fn try_from_info(info: &'a AccountView) -> Result<Self, Error> {
        if unlikely(!address_eq(unsafe { info.owner() }, &System::ID)) {
            return Err(ProgramError::InvalidAccountOwner.into());
        }

        Ok(SystemAccount { info })
    }
}

impl<'a> From<SystemAccount<'a>> for &'a AccountView {
    #[inline(always)]
    fn from(value: SystemAccount<'a>) -> Self {
        value.info
    }
}

impl AsRef<AccountView> for SystemAccount<'_> {
    #[inline(always)]
    fn as_ref(&self) -> &AccountView {
        self.info
    }
}

impl ReadableAccount for SystemAccount<'_> {
    type DataUnchecked = [u8];
    type Data<'a>
        = Ref<'a, [u8]>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.info.try_borrow().map_err(Into::into)
    }

    #[inline]
    fn data_unchecked(&self) -> Result<&Self::DataUnchecked, Error> {
        Ok(unsafe { self.info.borrow_unchecked() })
    }
}

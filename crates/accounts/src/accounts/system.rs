use {
    crate::{FromAccountInfo, ReadableAccount},
    pinocchio::{
        account_info::{AccountInfo, Ref},
        hint::unlikely,
        pubkey::pubkey_eq,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct SystemAccount<'a> {
    info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for SystemAccount<'a> {
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if unlikely(!pubkey_eq(info.owner(), &pinocchio_system::ID)) {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        Ok(SystemAccount { info })
    }
}

impl<'a> From<SystemAccount<'a>> for &'a AccountInfo {
    #[inline(always)]
    fn from(value: SystemAccount<'a>) -> Self {
        value.info
    }
}

impl AsRef<AccountInfo> for SystemAccount<'_> {
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
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
        self.info.try_borrow_data().map_err(Into::into)
    }

    #[inline]
    fn data_unchecked(&self) -> Result<&Self::DataUnchecked, Error> {
        Ok(unsafe { self.info.borrow_data_unchecked() })
    }
}

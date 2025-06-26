use {
    crate::{FromAccountInfo, ReadableAccount},
    pinocchio::account_info::{AccountInfo, Ref},
    typhoon_errors::{Error, ErrorCode},
};

pub struct SystemAccount<'a> {
    info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for SystemAccount<'a> {
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if !info.is_owned_by(&pinocchio_system::ID) {
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
    type Data<'a>
        = Ref<'a, [u8]>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.info.try_borrow_data().map_err(Into::into)
    }
}

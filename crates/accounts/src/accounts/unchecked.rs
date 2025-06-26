use {
    crate::{FromAccountInfo, ReadableAccount},
    pinocchio::account_info::{AccountInfo, Ref},
    typhoon_errors::Error,
};

pub struct UncheckedAccount<'a> {
    info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for UncheckedAccount<'a> {
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        Ok(UncheckedAccount { info })
    }
}

impl<'a> From<UncheckedAccount<'a>> for &'a AccountInfo {
    #[inline(always)]
    fn from(value: UncheckedAccount<'a>) -> Self {
        value.info
    }
}

impl AsRef<AccountInfo> for UncheckedAccount<'_> {
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl ReadableAccount for UncheckedAccount<'_> {
    type Data<'a>
        = Ref<'a, [u8]>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.info.try_borrow_data().map_err(Into::into)
    }
}

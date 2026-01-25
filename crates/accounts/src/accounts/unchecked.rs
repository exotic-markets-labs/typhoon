use {
    crate::{FromAccountInfo, ReadableAccount},
    solana_account_view::{AccountView, Ref},
    typhoon_errors::Error,
};

pub struct UncheckedAccount<'a> {
    info: &'a AccountView,
}

impl<'a> FromAccountInfo<'a> for UncheckedAccount<'a> {
    #[inline(always)]
    fn try_from_info(info: &'a AccountView) -> Result<Self, Error> {
        Ok(UncheckedAccount { info })
    }
}

impl<'a> From<UncheckedAccount<'a>> for &'a AccountView {
    #[inline(always)]
    fn from(value: UncheckedAccount<'a>) -> Self {
        value.info
    }
}

impl AsRef<AccountView> for UncheckedAccount<'_> {
    #[inline(always)]
    fn as_ref(&self) -> &AccountView {
        self.info
    }
}

impl ReadableAccount for UncheckedAccount<'_> {
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

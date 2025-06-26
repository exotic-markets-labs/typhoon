use {
    crate::{FromAccountInfo, ReadableAccount, SignerAccount},
    pinocchio::account_info::{AccountInfo, Ref},
    typhoon_errors::{Error, ErrorCode},
};

pub struct Signer<'a> {
    info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for Signer<'a> {
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if !info.is_signer() {
            return Err(ErrorCode::AccountNotSigner.into());
        }

        Ok(Signer { info })
    }
}

impl<'a> From<Signer<'a>> for &'a AccountInfo {
    #[inline(always)]
    fn from(value: Signer<'a>) -> Self {
        value.info
    }
}

impl AsRef<AccountInfo> for Signer<'_> {
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl SignerAccount for Signer<'_> {}

impl ReadableAccount for Signer<'_> {
    type Data<'a>
        = Ref<'a, [u8]>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.info.try_borrow_data().map_err(Into::into)
    }
}

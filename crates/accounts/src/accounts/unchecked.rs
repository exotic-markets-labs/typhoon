use {
    crate::{FromAccountInfo, ReadableAccount},
    pinocchio::{
        account_info::{AccountInfo, Ref},
        pubkey::Pubkey,
    },
    typhoon_errors::Error,
};

pub struct UncheckedAccount<'a> {
    info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for UncheckedAccount<'a> {
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        Ok(UncheckedAccount { info })
    }
}

impl<'a> From<UncheckedAccount<'a>> for &'a AccountInfo {
    fn from(value: UncheckedAccount<'a>) -> Self {
        value.info
    }
}

impl AsRef<AccountInfo> for UncheckedAccount<'_> {
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl ReadableAccount for UncheckedAccount<'_> {
    type Data<'a>
        = Ref<'a, [u8]>
    where
        Self: 'a;

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

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

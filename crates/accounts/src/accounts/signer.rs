use {
    crate::{FromAccountInfo, ReadableAccount, SignerAccount},
    pinocchio::{
        account_info::{AccountInfo, Ref},
        pubkey::Pubkey,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct Signer<'a> {
    info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for Signer<'a> {
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if !info.is_signer() {
            return Err(ErrorCode::AccountNotSigner.into());
        }

        Ok(Signer { info })
    }
}

impl<'a> From<Signer<'a>> for &'a AccountInfo {
    fn from(value: Signer<'a>) -> Self {
        value.info
    }
}

impl AsRef<AccountInfo> for Signer<'_> {
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

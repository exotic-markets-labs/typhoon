use {
    crate::{FromAccountInfo, ReadableAccount, SignerAccount},
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    typhoon_errors::Error,
};

pub struct Signer<'a> {
    info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for Signer<'a> {
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, ProgramError> {
        if !info.is_signer() {
            return Err(Error::AccountNotSigner.into());
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
    type DataType = [u8];

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.info.is_owned_by(owner)
    }

    fn lamports(&self) -> Result<Ref<u64>, ProgramError> {
        self.info.try_borrow_lamports()
    }

    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError> {
        self.info.try_borrow_data()
    }
}

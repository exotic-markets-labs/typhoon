use {
    crate::{FromAccountInfo, ReadableAccount},
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    typhoon_errors::Error,
};

pub struct SystemAccount<'a> {
    info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for SystemAccount<'a> {
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, ProgramError> {
        if !info.is_owned_by(&pinocchio_system::ID) {
            return Err(Error::AccountOwnedByWrongProgram.into());
        }

        Ok(SystemAccount { info })
    }
}

impl<'a> From<SystemAccount<'a>> for &'a AccountInfo {
    fn from(value: SystemAccount<'a>) -> Self {
        value.info
    }
}

impl AsRef<AccountInfo> for SystemAccount<'_> {
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl ReadableAccount for SystemAccount<'_> {
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

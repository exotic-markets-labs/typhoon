use {
    crate::{FromAccountInfo, ReadableAccount},
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

pub struct UncheckedAccount<'a> {
    info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for UncheckedAccount<'a> {
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, ProgramError> {
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

use {
    crate::{FromAccountInfo, ReadableAccount},
    typhoon_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref},
};

pub struct UncheckedAccount<'a> {
    info: &'a RawAccountInfo,
}

impl<'a> FromAccountInfo<'a> for UncheckedAccount<'a> {
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        Ok(UncheckedAccount { info })
    }
}

impl<'a> From<UncheckedAccount<'a>> for &'a RawAccountInfo {
    fn from(value: UncheckedAccount<'a>) -> Self {
        value.info
    }
}

impl AsRef<RawAccountInfo> for UncheckedAccount<'_> {
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl ReadableAccount for UncheckedAccount<'_> {
    type DataType = [u8];

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn owner(&self) -> &Pubkey {
        self.info.owner()
    }

    fn lamports(&self) -> Result<Ref<u64>, ProgramError> {
        self.info.try_borrow_lamports()
    }

    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError> {
        self.info.try_borrow_data()
    }
}

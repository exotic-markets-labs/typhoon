use {
    crate::{FromAccountInfo, ReadableAccount, SignerAccount},
    crayfish_errors::Error,
    crayfish_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref},
};

pub struct Signer<'a> {
    info: &'a RawAccountInfo,
}

impl<'a> FromAccountInfo<'a> for Signer<'a> {
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if !info.is_signer() {
            return Err(Error::AccountNotSigner.into());
        }

        Ok(Signer { info })
    }
}

impl<'a> AsRef<RawAccountInfo> for Signer<'a> {
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<'a> SignerAccount for Signer<'a> {}

impl<'a> ReadableAccount for Signer<'a> {
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

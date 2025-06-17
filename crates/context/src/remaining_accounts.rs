use {
    crate::HandlerContext,
    pinocchio::{account_info::AccountInfo, pubkey::Pubkey},
    typhoon_errors::Error,
};

impl<'a> HandlerContext<'a> for &'a [AccountInfo] {
    #[inline(always)]
    fn from_entrypoint(
        _program_id: &Pubkey,
        accounts: &mut &'a [AccountInfo],
        _instruction_data: &mut &'a [u8],
    ) -> Result<Self, Error> {
        Ok(accounts)
    }
}

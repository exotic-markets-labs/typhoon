use {
    crate::HandlerContext,
    pinocchio::{account_info::AccountInfo, pubkey::Pubkey},
    typhoon_errors::Error,
};

pub struct Remaining<'a>(pub &'a [AccountInfo]);

impl<'b> HandlerContext<'_, 'b, '_> for Remaining<'b> {
    #[inline(always)]
    fn from_entrypoint(
        _program_id: &Pubkey,
        accounts: &mut &'b [AccountInfo],
        _instruction_data: &mut &[u8],
    ) -> Result<Self, Error> {
        Ok(Remaining(accounts))
    }
}

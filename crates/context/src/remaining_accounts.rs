use {
    crate::HandlerContext, solana_account_view::AccountView, solana_address::Address,
    typhoon_errors::Error,
};

pub struct Remaining<'a>(pub &'a [AccountView]);

impl<'b> HandlerContext<'_, 'b, '_> for Remaining<'b> {
    #[inline(always)]
    fn from_entrypoint(
        _program_id: &Address,
        accounts: &mut &'b [AccountView],
        _instruction_data: &mut &[u8],
    ) -> Result<Self, Error> {
        Ok(Remaining(accounts))
    }
}

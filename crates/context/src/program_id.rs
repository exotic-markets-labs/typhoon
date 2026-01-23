use {crate::HandlerContext, solana_account_view::AccountView, solana_address::Address};

pub struct ProgramIdArg<'a>(pub &'a Address);

impl<'a> HandlerContext<'a, '_, '_> for ProgramIdArg<'a> {
    #[inline(always)]
    fn from_entrypoint(
        program_id: &'a Address,
        _accounts: &mut &[AccountView],
        _instruction_data: &mut &[u8],
    ) -> Result<Self, typhoon_errors::Error> {
        Ok(ProgramIdArg(program_id))
    }
}

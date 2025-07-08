use {
    crate::HandlerContext,
    pinocchio::{account_info::AccountInfo, pubkey::Pubkey},
};

pub struct ProgramIdArg<'a>(pub &'a Pubkey);

impl<'a> HandlerContext<'a, '_, '_> for ProgramIdArg<'a> {
    #[inline(always)]
    fn from_entrypoint(
        program_id: &'a Pubkey,
        _accounts: &mut &[AccountInfo],
        _instruction_data: &mut &[u8],
    ) -> Result<Self, typhoon_errors::Error> {
        Ok(ProgramIdArg(program_id))
    }
}

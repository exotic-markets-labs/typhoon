use {
    crate::HandlerContext,
    pinocchio::{account_info::AccountInfo, program_error::ProgramError},
};

impl<'a> HandlerContext<'a> for &'a [AccountInfo] {
    fn from_entrypoint(
        accounts: &mut &'a [AccountInfo],
        _instruction_data: &mut &'a [u8],
    ) -> Result<Self, ProgramError> {
        Ok(accounts)
    }
}

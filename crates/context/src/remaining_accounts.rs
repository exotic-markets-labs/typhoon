use {crate::HandlerContext, pinocchio::account_info::AccountInfo, typhoon_errors::Error};

impl<'a> HandlerContext<'a> for &'a [AccountInfo] {
    fn from_entrypoint(
        accounts: &mut &'a [AccountInfo],
        _instruction_data: &mut &'a [u8],
    ) -> Result<Self, Error> {
        Ok(accounts)
    }
}

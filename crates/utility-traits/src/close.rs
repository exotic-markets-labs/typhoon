use {
    pinocchio::program_error::ProgramError, typhoon_accounts::WritableAccount,
    typhoon_errors::Error,
};

pub trait CloseAccount: WritableAccount {
    #[inline(always)]
    fn close(&self, destination: &impl WritableAccount) -> Result<(), Error> {
        let dest_lamports = *destination.lamports()?;
        let source_lamports = *self.lamports()?;

        *destination.mut_lamports()? = dest_lamports
            .checked_add(source_lamports)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        *self.mut_lamports()? = 0;

        self.assign(&pinocchio_system::ID);
        self.resize(0)
    }
}

impl<T: WritableAccount> CloseAccount for T {}

use {
    typhoon_accounts::WritableAccount,
    typhoon_program::{program_error::ProgramError, system_program, ProgramResult},
};

pub trait CloseAccount: WritableAccount {
    fn close(&self, destination: &impl WritableAccount) -> ProgramResult {
        let dest_lamports = *destination.lamports()?;
        let source_lamports = *self.lamports()?;

        *destination.mut_lamports()? = dest_lamports
            .checked_add(source_lamports)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        *self.mut_lamports()? = 0;

        self.assign(&system_program::ID);
        self.realloc(0, false)
    }
}

impl<T: WritableAccount> CloseAccount for T {}

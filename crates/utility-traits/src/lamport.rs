use {
    typhoon_accounts::{Mut, Signer, SignerAccount, WritableAccount},
    typhoon_program::program_error::ProgramError,
};

pub trait Lamports: WritableAccount + SignerAccount {
    fn send(&self, to: &impl WritableAccount, amount: u64) -> Result<(), ProgramError> {
        let mut payer_lamports = self.mut_lamports()?;
        let mut recipient_lamports = to.mut_lamports()?;

        *payer_lamports = payer_lamports
            .checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        *recipient_lamports = recipient_lamports
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }

    fn send_all(&self, to: &impl WritableAccount) -> Result<(), ProgramError> {
        let amount = *self.lamports()?;
        let mut payer_lamports = self.mut_lamports()?;
        let mut recipient_lamports = to.mut_lamports()?;

        *payer_lamports = payer_lamports
            .checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        *recipient_lamports = recipient_lamports
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }
}

impl Lamports for Mut<Signer<'_>> {}

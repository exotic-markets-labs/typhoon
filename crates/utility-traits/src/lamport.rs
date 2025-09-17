use {
    pinocchio::program_error::ProgramError,
    typhoon_accounts::{Mut, Signer, SignerAccount, SystemAccount, WritableAccount},
    typhoon_errors::Error,
};

pub trait LamportsChecked: WritableAccount + SignerAccount {
    #[inline(always)]
    fn send(&self, to: &impl WritableAccount, amount: u64) -> Result<(), Error> {
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

    #[inline(always)]
    fn send_all(&self, to: &impl WritableAccount) -> Result<(), Error> {
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

impl LamportsChecked for Mut<Signer<'_, SystemAccount<'_>>> {}
impl LamportsChecked for Mut<Signer<'_>> {}

use {
    crate::{TokenAccount, TokenProgram},
    pinocchio_token::instructions::InitializeAccount3,
    typhoon_accounts::{
        Account, FromAccountInfo, Mut, ReadableAccount, SignerAccount, SystemAccount,
        WritableAccount,
    },
    typhoon_errors::Error,
    typhoon_program::{
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program::instructions::{Allocate, Assign, CreateAccount, Transfer},
        sysvars::rent::Rent,
        RawAccountInfo, SignerSeeds,
    },
    typhoon_traits::ProgramId,
};

pub trait TokenAccountTrait<'a>: WritableAccount + Into<&'a RawAccountInfo> {
    fn create_token_account(
        self,
        rent: &Rent,
        payer: &(impl WritableAccount + SignerAccount),
        mint: &impl ReadableAccount,
        owner: &Pubkey,
        seeds: Option<&[SignerSeeds]>,
    ) -> Result<Mut<Account<'a, TokenAccount>>, ProgramError> {
        let current_lamports = { *self.lamports()? };
        let space = TokenAccount::LEN;
        if current_lamports == 0 {
            let lamports = rent.minimum_balance(space);
            CreateAccount {
                from: payer.as_ref(),
                to: self.as_ref(),
                lamports,
                space: space as u64,
                owner: &TokenProgram::ID,
            }
            .invoke_signed(seeds.unwrap_or_default())?;
        } else {
            if payer.key() == self.key() {
                return Err(Error::TryingToInitPayerAsProgramAccount.into());
            }

            let required_lamports = rent
                .minimum_balance(space)
                .max(1)
                .saturating_sub(current_lamports);

            if required_lamports > 0 {
                Transfer {
                    from: payer.as_ref(),
                    to: self.as_ref(),
                    lamports: required_lamports,
                }
                .invoke()?;
            }

            Allocate {
                account: self.as_ref(),
                space: space as u64,
            }
            .invoke_signed(seeds.unwrap_or_default())?;

            Assign {
                account: self.as_ref(),
                owner: &TokenProgram::ID,
            }
            .invoke_signed(seeds.unwrap_or_default())?;
        }

        InitializeAccount3 {
            account: self.as_ref(),
            mint: mint.as_ref(),
            owner,
        }
        .invoke()?;

        Mut::try_from_info(self.into())
    }
}

impl<'a> TokenAccountTrait<'a> for Mut<SystemAccount<'a>> {}

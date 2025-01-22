use {
    crate::{TokenAccount, TokenProgram},
    pinocchio_token::instructions::InitializeAccount3,
    typhoon_accounts::{Mut, ReadableAccount, SignerAccount, SystemAccount, WritableAccount},
    typhoon_program::{
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program::instructions::{Allocate, Assign, CreateAccount, Transfer},
        sysvars::rent::Rent,
        SignerSeeds,
    },
    typhoon_traits::ProgramId,
};

pub trait TokenAccountTrait: WritableAccount {
    fn create_token_account(
        &self,
        rent: &Rent,
        payer: &(impl WritableAccount + SignerAccount),
        mint: &impl ReadableAccount,
        owner: &Pubkey,
        signer_seeds: Option<SignerSeeds>,
    ) -> Result<(), ProgramError> {
        let current_lamports = self.lamports()?;
        let space = TokenAccount::LEN;
        let signers: &[SignerSeeds] = match signer_seeds {
            Some(seeds) => &[seeds],
            None => &[],
        };

        if *current_lamports == 0 {
            let lamports = rent.minimum_balance(space);
            CreateAccount {
                from: payer.as_ref(),
                to: self.as_ref(),
                lamports,
                space: space as u64,
                owner: &TokenProgram::ID,
            }
            .invoke_signed(signers)?;
        } else {
            let required_lamports = rent
                .minimum_balance(space)
                .max(1)
                .saturating_sub(*current_lamports);

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
            .invoke_signed(signers)?;

            Assign {
                account: self.as_ref(),
                owner: &TokenProgram::ID,
            }
            .invoke_signed(signers)?;
        }

        InitializeAccount3 {
            account: self.as_ref(),
            mint: mint.as_ref(),
            owner,
        }
        .invoke()?;

        Ok(())
    }
}

impl TokenAccountTrait for Mut<SystemAccount<'_>> {}

use {
    typhoon_accounts::{
        Account, Discriminator, FromAccountInfo, Mut, Owner, RefFromBytes, Signer as SignerAccount,
        SystemAccount, WritableAccount,
    },
    typhoon_program::{
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program::instructions::{Allocate, Assign, Transfer},
        sysvars::rent::Rent,
        RawAccountInfo, SignerSeeds,
    },
    typhoon_utility::create_or_assign,
};

pub trait SystemCpi<'a>: WritableAccount + Into<&'a RawAccountInfo>
where
    Self: Sized,
{
    fn allocate(&self, new_space: u64) -> Result<(), ProgramError> {
        Allocate {
            account: self.as_ref(),
            space: new_space,
        }
        .invoke()
    }

    fn assign(&self, owner: &Pubkey) -> Result<(), ProgramError> {
        Assign {
            account: self.as_ref(),
            owner,
        }
        .invoke()
    }

    fn create_account<T: Discriminator + RefFromBytes + Owner>(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        owner: &Pubkey,
        space: usize,
        seeds: Option<&[SignerSeeds]>,
    ) -> Result<Mut<Account<'a, T>>, ProgramError> {
        create_or_assign(&self, rent, payer, owner, space, seeds)?;

        // Set discriminator
        {
            let mut data = self.as_ref().try_borrow_mut_data()?;
            data[..T::DISCRIMINATOR.len()].copy_from_slice(T::DISCRIMINATOR);
        }

        Mut::try_from_info(self.into())
    }

    fn transfer(&self, to: &impl WritableAccount, amount: u64) -> Result<(), ProgramError> {
        Transfer {
            from: self.as_ref(),
            lamports: amount,
            to: to.as_ref(),
        }
        .invoke()
    }
}

impl<'a> SystemCpi<'a> for Mut<SystemAccount<'a>> {}
impl<'a> SystemCpi<'a> for Mut<SignerAccount<'a>> {}

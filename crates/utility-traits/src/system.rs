use {
    pinocchio::{
        account_info::AccountInfo, instruction::Signer, pubkey::Pubkey, sysvars::rent::Rent,
    },
    pinocchio_system::instructions::{Allocate, Assign, Transfer},
    typhoon_accounts::{
        Account, Discriminator, FromAccountInfo, Mut, Owner, RefFromBytes, Signer as SignerAccount,
        SystemAccount, UncheckedAccount, WritableAccount,
    },
    typhoon_errors::Error,
    typhoon_utility::create_or_assign,
};

pub trait SystemCpi<'a>: WritableAccount + Into<&'a AccountInfo>
where
    Self: Sized,
{
    fn allocate(&self, new_space: u64) -> Result<(), Error> {
        Allocate {
            account: self.as_ref(),
            space: new_space,
        }
        .invoke()
        .map_err(Into::into)
    }

    fn assign(&self, owner: &Pubkey) -> Result<(), Error> {
        Assign {
            account: self.as_ref(),
            owner,
        }
        .invoke()
        .map_err(Into::into)
    }

    fn create_account<T: Discriminator + RefFromBytes + Owner>(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        owner: &Pubkey,
        space: usize,
        seeds: Option<&[Signer]>,
    ) -> Result<Mut<Account<'a, T>>, Error> {
        create_or_assign(&self, rent, payer, owner, space, seeds)?;

        // Set discriminator
        {
            let mut data = self.as_ref().try_borrow_mut_data()?;
            data[..T::DISCRIMINATOR.len()].copy_from_slice(T::DISCRIMINATOR);
        }

        Mut::try_from_info(self.into())
    }

    fn transfer(&self, to: &impl WritableAccount, amount: u64) -> Result<(), Error> {
        Transfer {
            from: self.as_ref(),
            lamports: amount,
            to: to.as_ref(),
        }
        .invoke()
        .map_err(Into::into)
    }
}

impl<'a> SystemCpi<'a> for Mut<SystemAccount<'a>> {}
impl<'a> SystemCpi<'a> for Mut<SignerAccount<'a>> {}
impl<'a> SystemCpi<'a> for Mut<UncheckedAccount<'a>> {}

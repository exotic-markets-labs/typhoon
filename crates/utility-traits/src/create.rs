use {
    pinocchio::{account_info::AccountInfo, instruction, pubkey::Pubkey, sysvars::rent::Rent},
    typhoon_accounts::{Account, Discriminator, Mut, RefFromBytes, SystemAccount, WritableAccount},
    typhoon_errors::Error,
    typhoon_utility::create_or_assign,
};

pub trait CreateAccountCpi<'a, T: Discriminator + RefFromBytes> {
    fn create(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        owner: &Pubkey,
        space: usize,
        seeds: Option<&[instruction::Signer]>,
    ) -> Result<Mut<Account<'a, T>>, Error>;
}

impl<'a, T> CreateAccountCpi<'a, T> for &'a AccountInfo
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn create(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        owner: &Pubkey,
        space: usize,
        seeds: Option<&[instruction::Signer]>,
    ) -> Result<Mut<Account<'a, T>>, Error> {
        create_or_assign(self, rent, payer, owner, space, seeds)?;

        // Set discriminator
        {
            let data = self.data_ptr();
            unsafe {
                core::ptr::copy_nonoverlapping(
                    T::DISCRIMINATOR.as_ptr(),
                    data,
                    T::DISCRIMINATOR.len(),
                );
            }
        }

        Ok(Mut::from_raw_info(self))
    }
}

impl<'a, T> CreateAccountCpi<'a, T> for Mut<SystemAccount<'a>>
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn create(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        owner: &Pubkey,
        space: usize,
        seeds: Option<&[instruction::Signer]>,
    ) -> Result<Mut<Account<'a, T>>, Error> {
        create_or_assign(self.as_ref(), rent, payer, owner, space, seeds)?;

        // Set discriminator
        {
            let mut data = self.as_ref().try_borrow_mut_data()?;
            data[..T::DISCRIMINATOR.len()].copy_from_slice(T::DISCRIMINATOR);
        }

        Ok(Mut::from_raw_info(self.into()))
    }
}

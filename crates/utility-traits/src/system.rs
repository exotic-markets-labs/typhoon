use {
    pinocchio::{account_info::AccountInfo, pubkey::Pubkey},
    pinocchio_system::instructions::{Allocate, Assign, Transfer},
    typhoon_accounts::{
        Mut, Signer as SignerAccount, SystemAccount, UncheckedAccount, WritableAccount,
    },
    typhoon_errors::Error,
};

pub trait SystemCpi<'a>: WritableAccount + Into<&'a AccountInfo>
where
    Self: Sized,
{
    #[inline(always)]
    fn allocate(&self, new_space: u64) -> Result<(), Error> {
        Allocate {
            account: self.as_ref(),
            space: new_space,
        }
        .invoke()
        .map_err(Into::into)
    }

    #[inline(always)]
    fn assign(&self, owner: &Pubkey) -> Result<(), Error> {
        Assign {
            account: self.as_ref(),
            owner,
        }
        .invoke()
        .map_err(Into::into)
    }

    #[inline(always)]
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

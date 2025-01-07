use {
    typhoon_accounts::{
        FromAccountInfo, Mut, ReadableAccount, Signer as SignerAccount, SystemAccount,
        WritableAccount,
    },
    typhoon_program::{
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program::instructions::{Allocate, Assign, CreateAccount, Transfer},
        sysvars::{rent::Rent, Sysvar},
        RawAccountInfo, SignerSeeds,
    },
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

    fn create_account<T: ReadableAccount + FromAccountInfo<'a>>(
        self,
        payer: &impl ReadableAccount,
        owner: &Pubkey,
        space: u64,
        seeds: Option<&[SignerSeeds]>,
    ) -> Result<Mut<T>, ProgramError> {
        CreateAccount {
            from: payer.as_ref(),
            lamports: Rent::get()?.minimum_balance(space as usize),
            owner,
            space,
            to: self.as_ref(),
        }
        .invoke_signed(seeds.unwrap_or_default())?;

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

use {
    crate::{TokenAccount, TokenProgram},
    pinocchio::{
        account_info::AccountInfo, instruction::Signer, pubkey::Pubkey, sysvars::rent::Rent,
    },
    pinocchio_associated_token_account::instructions::{Create, CreateIdempotent},
    pinocchio_token::instructions::InitializeAccount3,
    typhoon_accounts::{
        Account, FromAccountInfo, InterfaceAccount, Mut, ProgramId, ReadableAccount, SystemAccount,
        UncheckedAccount, WritableAccount,
    },
    typhoon_errors::Error,
    typhoon_utility::create_or_assign,
};

pub trait SplCreateToken<'a, T>
where
    T: ReadableAccount + FromAccountInfo<'a>,
{
    fn create_token_account(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &Pubkey,
        seeds: Option<&[Signer]>,
    ) -> Result<Mut<T>, Error>;

    fn create_associated_token_account(
        self,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &impl ReadableAccount,
        system_program: &impl ReadableAccount,
        token_program: &impl ReadableAccount,
    ) -> Result<Mut<T>, Error>;

    fn create_idempotent_associated_token_account(
        self,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &impl ReadableAccount,
        system_program: &impl ReadableAccount,
        token_program: &impl ReadableAccount,
    ) -> Result<Mut<T>, Error>;
}

impl<'a> SplCreateToken<'a, Account<'a, TokenAccount>> for &'a AccountInfo {
    fn create_token_account(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &Pubkey,
        seeds: Option<&[Signer]>,
    ) -> Result<Mut<Account<'a, TokenAccount>>, Error> {
        create_or_assign(
            self,
            rent,
            payer,
            &TokenProgram::ID,
            TokenAccount::LEN,
            seeds,
        )?;

        InitializeAccount3 {
            account: self,
            mint: mint.as_ref(),
            owner,
        }
        .invoke()?;

        Ok(Mut::from_raw_info(self))
    }

    fn create_associated_token_account(
        self,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &impl ReadableAccount,
        system_program: &impl ReadableAccount,
        token_program: &impl ReadableAccount,
    ) -> Result<Mut<Account<'a, TokenAccount>>, Error> {
        Create {
            funding_account: payer.as_ref(),
            account: self,
            wallet: owner.as_ref(),
            mint: mint.as_ref(),
            system_program: system_program.as_ref(),
            token_program: token_program.as_ref(),
        }
        .invoke()?;

        Ok(Mut::from_raw_info(self))
    }

    fn create_idempotent_associated_token_account(
        self,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &impl ReadableAccount,
        system_program: &impl ReadableAccount,
        token_program: &impl ReadableAccount,
    ) -> Result<Mut<Account<'a, TokenAccount>>, Error> {
        CreateIdempotent {
            funding_account: payer.as_ref(),
            account: self,
            wallet: owner.as_ref(),
            mint: mint.as_ref(),
            system_program: system_program.as_ref(),
            token_program: token_program.as_ref(),
        }
        .invoke()?;

        Ok(Mut::from_raw_info(self))
    }
}

impl<'a> SplCreateToken<'a, InterfaceAccount<'a, TokenAccount>> for &'a AccountInfo {
    fn create_token_account(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &Pubkey,
        seeds: Option<&[Signer]>,
    ) -> Result<Mut<InterfaceAccount<'a, TokenAccount>>, Error> {
        create_or_assign(
            self,
            rent,
            payer,
            &TokenProgram::ID,
            TokenAccount::LEN,
            seeds,
        )?;

        InitializeAccount3 {
            account: self,
            mint: mint.as_ref(),
            owner,
        }
        .invoke()?;

        Ok(Mut::from_raw_info(self))
    }

    fn create_associated_token_account(
        self,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &impl ReadableAccount,
        system_program: &impl ReadableAccount,
        token_program: &impl ReadableAccount,
    ) -> Result<Mut<InterfaceAccount<'a, TokenAccount>>, Error> {
        Create {
            funding_account: payer.as_ref(),
            account: self,
            wallet: owner.as_ref(),
            mint: mint.as_ref(),
            system_program: system_program.as_ref(),
            token_program: token_program.as_ref(),
        }
        .invoke()?;

        Ok(Mut::from_raw_info(self))
    }

    fn create_idempotent_associated_token_account(
        self,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &impl ReadableAccount,
        system_program: &impl ReadableAccount,
        token_program: &impl ReadableAccount,
    ) -> Result<Mut<InterfaceAccount<'a, TokenAccount>>, Error> {
        CreateIdempotent {
            funding_account: payer.as_ref(),
            account: self,
            wallet: owner.as_ref(),
            mint: mint.as_ref(),
            system_program: system_program.as_ref(),
            token_program: token_program.as_ref(),
        }
        .invoke()?;

        Ok(Mut::from_raw_info(self))
    }
}

macro_rules! impl_trait {
    ($target: ident, $origin: ident) => {
        impl<'a> SplCreateToken<'a, $target<'a, TokenAccount>> for Mut<$origin<'a>> {
            fn create_token_account(
                self,
                rent: &Rent,
                payer: &impl WritableAccount,
                mint: &impl ReadableAccount,
                owner: &Pubkey,
                seeds: Option<&[Signer]>,
            ) -> Result<Mut<$target<'a, TokenAccount>>, Error> {
                create_or_assign(
                    self.as_ref(),
                    rent,
                    payer,
                    &TokenProgram::ID,
                    TokenAccount::LEN,
                    seeds,
                )?;

                InitializeAccount3 {
                    account: self.as_ref(),
                    mint: mint.as_ref(),
                    owner,
                }
                .invoke()?;

                Ok(Mut::from_raw_info(self.into()))
            }

            fn create_associated_token_account(
                self,
                payer: &impl WritableAccount,
                mint: &impl ReadableAccount,
                owner: &impl ReadableAccount,
                system_program: &impl ReadableAccount,
                token_program: &impl ReadableAccount,
            ) -> Result<Mut<$target<'a, TokenAccount>>, Error> {
                Create {
                    funding_account: payer.as_ref(),
                    account: self.as_ref(),
                    wallet: owner.as_ref(),
                    mint: mint.as_ref(),
                    system_program: system_program.as_ref(),
                    token_program: token_program.as_ref(),
                }
                .invoke()?;

                Ok(Mut::from_raw_info(self.into()))
            }

            fn create_idempotent_associated_token_account(
                self,
                payer: &impl WritableAccount,
                mint: &impl ReadableAccount,
                owner: &impl ReadableAccount,
                system_program: &impl ReadableAccount,
                token_program: &impl ReadableAccount,
            ) -> Result<Mut<$target<'a, TokenAccount>>, Error> {
                CreateIdempotent {
                    funding_account: payer.as_ref(),
                    account: self.as_ref(),
                    wallet: owner.as_ref(),
                    mint: mint.as_ref(),
                    system_program: system_program.as_ref(),
                    token_program: token_program.as_ref(),
                }
                .invoke()?;

                Ok(Mut::from_raw_info(self.into()))
            }
        }
    };
}

impl_trait!(Account, SystemAccount);
impl_trait!(InterfaceAccount, SystemAccount);
impl_trait!(Account, UncheckedAccount);
impl_trait!(InterfaceAccount, UncheckedAccount);

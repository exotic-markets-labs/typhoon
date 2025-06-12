use {
    crate::{
        ata_instructions::{Create, CreateIdempotent},
        spl_instructions::{InitializeAccount3, InitializeMint2},
        Mint, TokenAccount, TokenProgram,
    },
    pinocchio::{
        account_info::AccountInfo, instruction::Signer, pubkey::Pubkey, sysvars::rent::Rent,
    },
    typhoon_accounts::{
        Account, FromAccountInfo, InterfaceAccount, Mut, ProgramId, ReadableAccount, SystemAccount,
        UncheckedAccount, WritableAccount,
    },
    typhoon_errors::Error,
    typhoon_utility::create_or_assign,
};

pub trait SplCreateToken<'a, T>: WritableAccount + Into<&'a AccountInfo>
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
    ) -> Result<Mut<T>, Error> {
        create_or_assign(
            &self,
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

        Mut::try_from_info(self.into())
    }

    fn create_associated_token_account(
        self,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &impl ReadableAccount,
        system_program: &impl ReadableAccount,
        token_program: &impl ReadableAccount,
    ) -> Result<Mut<T>, Error> {
        Create {
            funding_account: payer.as_ref(),
            account: self.as_ref(),
            wallet: owner.as_ref(),
            mint: mint.as_ref(),
            system_program: system_program.as_ref(),
            token_program: token_program.as_ref(),
        }
        .invoke()?;

        Mut::try_from_info(self.into())
    }

    fn create_idempotent_associated_token_account(
        self,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &impl ReadableAccount,
        system_program: &impl ReadableAccount,
        token_program: &impl ReadableAccount,
    ) -> Result<Mut<T>, Error> {
        CreateIdempotent {
            funding_account: payer.as_ref(),
            account: self.as_ref(),
            wallet: owner.as_ref(),
            mint: mint.as_ref(),
            system_program: system_program.as_ref(),
            token_program: token_program.as_ref(),
        }
        .invoke()?;

        Mut::try_from_info(self.into())
    }
}

pub trait SplCreateMint<'a, T>: WritableAccount + Into<&'a AccountInfo>
where
    T: ReadableAccount + FromAccountInfo<'a>,
{
    fn create_mint(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        mint_authority: &Pubkey,
        decimals: u8,
        freeze_authority: Option<&Pubkey>,
        seeds: Option<&[Signer]>,
    ) -> Result<Mut<T>, Error> {
        create_or_assign(&self, rent, payer, &TokenProgram::ID, Mint::LEN, seeds)?;

        InitializeMint2 {
            mint: self.as_ref(),
            mint_authority,
            decimals,
            freeze_authority,
        }
        .invoke_signed(seeds.unwrap_or_default())?;

        Mut::try_from_info(self.into())
    }
}

impl<'a> SplCreateToken<'a, Account<'a, TokenAccount>> for Mut<SystemAccount<'a>> {}
impl<'a> SplCreateToken<'a, InterfaceAccount<'a, TokenAccount>> for Mut<SystemAccount<'a>> {}
impl<'a> SplCreateToken<'a, Account<'a, TokenAccount>> for Mut<UncheckedAccount<'a>> {}
impl<'a> SplCreateToken<'a, InterfaceAccount<'a, TokenAccount>> for Mut<UncheckedAccount<'a>> {}
impl<'a> SplCreateMint<'a, Account<'a, Mint>> for Mut<SystemAccount<'a>> {}
impl<'a> SplCreateMint<'a, InterfaceAccount<'a, Mint>> for Mut<SystemAccount<'a>> {}
impl<'a> SplCreateMint<'a, Account<'a, Mint>> for Mut<UncheckedAccount<'a>> {}
impl<'a> SplCreateMint<'a, InterfaceAccount<'a, Mint>> for Mut<UncheckedAccount<'a>> {}

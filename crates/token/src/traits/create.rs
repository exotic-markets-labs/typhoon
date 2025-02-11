use {
    crate::{
        spl_instructions::{InitializeAccount3, InitializeMint2},
        Mint, TokenAccount, TokenProgram,
    },
    typhoon_accounts::{
        Account, FromAccountInfo, Mut, ProgramId, ReadableAccount, SystemAccount, WritableAccount,
    },
    typhoon_program::{
        program_error::ProgramError, pubkey::Pubkey, sysvars::rent::Rent, RawAccountInfo,
        SignerSeeds,
    },
    typhoon_utility::create_or_assign,
};

pub trait SPLCreate<'a>: WritableAccount + Into<&'a RawAccountInfo> {
    fn create_token_account(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        mint: &impl ReadableAccount,
        owner: &Pubkey,
        seeds: Option<&[SignerSeeds]>,
    ) -> Result<Mut<Account<'a, TokenAccount>>, ProgramError> {
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

    fn create_mint(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        mint_authority: &Pubkey,
        decimals: u8,
        freeze_authority: Option<&Pubkey>,
        seeds: Option<&[SignerSeeds]>,
    ) -> Result<Mut<Account<'a, Mint>>, ProgramError> {
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

impl<'a> SPLCreate<'a> for Mut<SystemAccount<'a>> {}

use {
    crate::{Mint, TokenProgram},
    pinocchio::{
        account_info::AccountInfo, instruction::Signer, pubkey::Pubkey, sysvars::rent::Rent,
    },
    pinocchio_token::instructions::InitializeMint2,
    typhoon_accounts::{
        Account, FromAccountInfo, InterfaceAccount, Mut, ProgramId, ReadableAccount, SystemAccount,
        UncheckedAccount, WritableAccount,
    },
    typhoon_errors::Error,
    typhoon_utility::create_or_assign,
};

pub trait SplCreateMint<'a, T>
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
    ) -> Result<Mut<T>, Error>;
}

impl<'a> SplCreateMint<'a, Account<'a, Mint>> for &'a AccountInfo {
    fn create_mint(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        mint_authority: &Pubkey,
        decimals: u8,
        freeze_authority: Option<&Pubkey>,
        seeds: Option<&[Signer]>,
    ) -> Result<Mut<Account<'a, Mint>>, Error> {
        create_or_assign(self, rent, payer, &TokenProgram::ID, Mint::LEN, seeds)?;

        InitializeMint2 {
            mint: self,
            mint_authority,
            decimals,
            freeze_authority,
        }
        .invoke_signed(seeds.unwrap_or_default())?;

        Mut::try_from_info(self)
    }
}

impl<'a> SplCreateMint<'a, InterfaceAccount<'a, Mint>> for &'a AccountInfo {
    fn create_mint(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        mint_authority: &Pubkey,
        decimals: u8,
        freeze_authority: Option<&Pubkey>,
        seeds: Option<&[Signer]>,
    ) -> Result<Mut<InterfaceAccount<'a, Mint>>, Error> {
        create_or_assign(self, rent, payer, &TokenProgram::ID, Mint::LEN, seeds)?;

        InitializeMint2 {
            mint: self,
            mint_authority,
            decimals,
            freeze_authority,
        }
        .invoke_signed(seeds.unwrap_or_default())?;

        Mut::try_from_info(self)
    }
}

macro_rules! impl_trait {
    ($target: ident, $origin: ident) => {
        impl<'a> SplCreateMint<'a, $target<'a, Mint>> for Mut<$origin<'a>> {
            fn create_mint(
                self,
                rent: &Rent,
                payer: &impl WritableAccount,
                mint_authority: &Pubkey,
                decimals: u8,
                freeze_authority: Option<&Pubkey>,
                seeds: Option<&[Signer]>,
            ) -> Result<Mut<$target<'a, Mint>>, Error> {
                create_or_assign(
                    self.as_ref(),
                    rent,
                    payer,
                    &TokenProgram::ID,
                    Mint::LEN,
                    seeds,
                )?;

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
    };
}

impl_trait!(Account, SystemAccount);
impl_trait!(InterfaceAccount, SystemAccount);
impl_trait!(Account, UncheckedAccount);
impl_trait!(InterfaceAccount, UncheckedAccount);

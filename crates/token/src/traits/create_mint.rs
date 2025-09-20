use {
    crate::{Mint, TokenProgram},
    pinocchio::{
        account_info::AccountInfo, instruction::Signer as CpiSigner, pubkey::Pubkey,
        sysvars::rent::Rent,
    },
    pinocchio_token::instructions::InitializeMint2,
    typhoon_accounts::{
        Account, FromAccountInfo, InterfaceAccount, Mut, ProgramId, ReadableAccount, Signer,
        SignerCheck, SystemAccount, UncheckedAccount, WritableAccount,
    },
    typhoon_errors::Error,
    typhoon_utility::create_or_assign,
};

pub trait SplCreateMint<'a, T: ReadableAccount>
where
    Self: Sized + Into<&'a AccountInfo>,
    T: ReadableAccount + FromAccountInfo<'a>,
{
    #[inline]
    fn create_mint(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        mint_authority: &Pubkey,
        decimals: u8,
        freeze_authority: Option<&Pubkey>,
        seeds: Option<&[CpiSigner]>,
    ) -> Result<Mut<T>, Error> {
        let info = self.into();
        create_or_assign(info, rent, payer, &TokenProgram::ID, Mint::LEN, seeds)?;

        InitializeMint2 {
            mint: info,
            mint_authority,
            decimals,
            freeze_authority,
        }
        .invoke_signed(seeds.unwrap_or_default())?;

        Mut::try_from_info(info)
    }
}

macro_rules! impl_trait {
    ($origin: ty) => {
        impl<'a> SplCreateMint<'a, Account<'a, Mint>> for $origin {}
        impl<'a, C> SplCreateMint<'a, Signer<'a, Account<'a, Mint>, C>> for $origin where
            C: SignerCheck
        {
        }
        impl<'a> SplCreateMint<'a, InterfaceAccount<'a, Mint>> for $origin {}
        impl<'a, C> SplCreateMint<'a, Signer<'a, InterfaceAccount<'a, Mint>, C>> for $origin where
            C: SignerCheck
        {
        }
    };
}

impl_trait!(&'a AccountInfo);
impl_trait!(SystemAccount<'a>);
impl_trait!(UncheckedAccount<'a>);

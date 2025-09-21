use {
    pinocchio::{account_info::AccountInfo, instruction, pubkey::Pubkey, sysvars::rent::Rent},
    typhoon_accounts::{
        Account, Discriminator, FromRaw, Mut, ReadableAccount, RefFromBytes, Signer, SignerCheck,
        SystemAccount, UncheckedAccount, WritableAccount,
    },
    typhoon_errors::Error,
    typhoon_utility::create_or_assign,
};

pub trait CreateAccountCpi<'a, T>
where
    Self: Sized + Into<&'a AccountInfo>,
    T: ReadableAccount + FromRaw<'a>,
{
    type D: Discriminator;

    #[inline(always)]
    fn create(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        owner: &Pubkey,
        space: usize,
        seeds: Option<&[instruction::Signer]>,
    ) -> Result<Mut<T>, Error> {
        let info = self.into();
        create_or_assign(info, rent, payer, owner, space, seeds)?;

        {
            let data = info.data_ptr();
            unsafe {
                core::ptr::copy_nonoverlapping(
                    Self::D::DISCRIMINATOR.as_ptr(),
                    data,
                    Self::D::DISCRIMINATOR.len(),
                );
            }
        }

        Ok(Mut::from_raw_info(info))
    }
}

macro_rules! impl_trait {
    ($origin: ty) => {
        impl<'a, T, C> CreateAccountCpi<'a, Signer<'a, Account<'a, T>, C>> for $origin
        where
            T: Discriminator + RefFromBytes,
            C: SignerCheck,
        {
            type D = T;
        }
        impl<'a, T> CreateAccountCpi<'a, Account<'a, T>> for $origin
        where
            T: Discriminator + RefFromBytes,
        {
            type D = T;
        }
    };
}

impl_trait!(&'a AccountInfo);
impl_trait!(SystemAccount<'a>);
impl_trait!(UncheckedAccount<'a>);

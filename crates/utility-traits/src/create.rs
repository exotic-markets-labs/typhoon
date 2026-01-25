use {
    pinocchio::{cpi, sysvars::rent::Rent, AccountView, Address},
    typhoon_accounts::{
        Account, FromRaw, Mut, ReadableAccount, RefFromBytes, Signer, SignerCheck, SystemAccount,
        UncheckedAccount, WritableAccount,
    },
    typhoon_errors::Error,
    typhoon_traits::Discriminator,
    typhoon_utility::create_or_assign,
};

pub trait CreateAccountCpi<'a, T>
where
    Self: Sized + Into<&'a AccountView>,
    T: ReadableAccount + FromRaw<'a>,
{
    type D: Discriminator;

    #[inline(always)]
    fn create(
        self,
        rent: &Rent,
        payer: &impl WritableAccount,
        owner: &Address,
        space: usize,
        seeds: Option<&[cpi::Signer]>,
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

impl_trait!(&'a AccountView);
impl_trait!(SystemAccount<'a>);
impl_trait!(UncheckedAccount<'a>);

use {
    crate::{FromAccountInfo, FromRaw, ReadableAccount, SignerAccount, UncheckedAccount},
    core::{marker::PhantomData, ops::Deref},
    pinocchio::account_info::AccountInfo,
    typhoon_errors::{Error, ErrorCode},
};

pub type SignerNoCheck<'a, T> = Signer<'a, T, NoCheck>;

pub trait SignerCheck {
    fn check(_info: &AccountInfo) -> Result<(), Error> {
        Ok(())
    }
}

pub struct Check;

impl SignerCheck for Check {
    fn check(info: &AccountInfo) -> Result<(), Error> {
        if info.is_signer() {
            Ok(())
        } else {
            Err(ErrorCode::AccountNotSigner.into())
        }
    }
}

pub struct NoCheck;

impl SignerCheck for NoCheck {}

pub struct Signer<'a, T = UncheckedAccount<'a>, C = Check>
where
    T: ReadableAccount,
    C: SignerCheck,
{
    pub(crate) acc: T,
    _phantom: PhantomData<&'a C>,
}

impl<'a, T, C> FromAccountInfo<'a> for Signer<'a, T, C>
where
    C: SignerCheck,
    T: ReadableAccount + FromAccountInfo<'a>,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        C::check(info)?;

        Ok(Signer {
            acc: T::try_from_info(info)?,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T, C> From<Signer<'a, T, C>> for &'a AccountInfo
where
    C: SignerCheck,
    T: ReadableAccount + Into<&'a AccountInfo>,
{
    #[inline(always)]
    fn from(value: Signer<'a, T, C>) -> Self {
        value.acc.into()
    }
}

impl<T, C> AsRef<AccountInfo> for Signer<'_, T, C>
where
    C: SignerCheck,
    T: ReadableAccount,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.acc.as_ref()
    }
}

impl<T, C> SignerAccount for Signer<'_, T, C>
where
    T: ReadableAccount,
    C: SignerCheck,
{
}

impl<T, C> Deref for Signer<'_, T, C>
where
    C: SignerCheck,
    T: ReadableAccount,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.acc
    }
}

impl<T, C> ReadableAccount for Signer<'_, T, C>
where
    C: SignerCheck,
    T: ReadableAccount,
{
    type DataUnchecked = T::DataUnchecked;
    type Data<'a>
        = T::Data<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.acc.data()
    }

    #[inline]
    fn data_unchecked(&self) -> Result<&Self::DataUnchecked, Error> {
        self.acc.data_unchecked()
    }
}

impl<'a, T, C> FromRaw<'a> for Signer<'a, T, C>
where
    T: ReadableAccount + FromRaw<'a>,
    C: SignerCheck,
{
    fn from_raw(info: &'a AccountInfo) -> Self {
        Self {
            acc: T::from_raw(info),
            _phantom: PhantomData,
        }
    }
}

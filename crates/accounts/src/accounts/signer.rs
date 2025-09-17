use {
    crate::{FromAccountInfo, ReadableAccount, SignerAccount, UncheckedAccount},
    core::marker::PhantomData,
    pinocchio::account_info::AccountInfo,
    typhoon_errors::{Error, ErrorCode},
};

pub struct Signer<'a, T = UncheckedAccount<'a>>
where
    T: ReadableAccount,
{
    pub(crate) acc: T,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> FromAccountInfo<'a> for Signer<'a, T>
where
    T: ReadableAccount + FromAccountInfo<'a>,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if !info.is_signer() {
            return Err(ErrorCode::AccountNotSigner.into());
        }

        Ok(Signer {
            acc: T::try_from_info(info)?,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Signer<'a, T>> for &'a AccountInfo
where
    T: ReadableAccount + Into<&'a AccountInfo>,
{
    #[inline(always)]
    fn from(value: Signer<'a, T>) -> Self {
        value.acc.into()
    }
}

impl<T> AsRef<AccountInfo> for Signer<'_, T>
where
    T: ReadableAccount,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.acc.as_ref()
    }
}

impl<T> SignerAccount for Signer<'_, T> where T: ReadableAccount {}

impl<T> ReadableAccount for Signer<'_, T>
where
    T: ReadableAccount,
{
    type Data<'a>
        = T::Data<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.acc.data()
    }
}

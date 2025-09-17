use {
    super::{Account, SystemAccount, UncheckedAccount},
    crate::{
        Discriminator, FromAccountInfo, FromRaw, InterfaceAccount, ReadableAccount, RefFromBytes,
        Signer, SignerAccount, WritableAccount,
    },
    pinocchio::{
        account_info::{AccountInfo, RefMut},
        program_error::ProgramError,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct Mut<T: ReadableAccount>(pub(crate) T);

impl<'a, T> FromAccountInfo<'a> for Mut<T>
where
    T: FromAccountInfo<'a> + ReadableAccount,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if !info.is_writable() {
            return Err(ErrorCode::AccountNotMutable.into());
        }

        Ok(Mut(T::try_from_info(info)?))
    }
}

impl<T> AsRef<AccountInfo> for Mut<T>
where
    T: ReadableAccount,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.0.as_ref()
    }
}

impl<'a, T> From<Mut<T>> for &'a AccountInfo
where
    T: ReadableAccount + Into<&'a AccountInfo>,
{
    #[inline(always)]
    fn from(value: Mut<T>) -> Self {
        value.0.into()
    }
}

impl<T> ReadableAccount for Mut<T>
where
    T: ReadableAccount,
{
    type Data<'a>
        = T::Data<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        self.0.data()
    }
}

macro_rules! impl_writable {
    ($name: ident) => {
        impl WritableAccount for Mut<$name<'_>> {
            type DataMut<'a>
                = RefMut<'a, [u8]>
            where
                Self: 'a;

            #[inline(always)]
            fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
                self.0.as_ref().try_borrow_mut_data().map_err(Into::into)
            }
        }
    };
}

impl_writable!(SystemAccount);
impl_writable!(UncheckedAccount);

macro_rules! impl_writable_signer {
    ($name: ident) => {
        impl WritableAccount for Mut<Signer<'_, $name<'_>>> {
            type DataMut<'a>
                = RefMut<'a, [u8]>
            where
                Self: 'a;
            #[inline(always)]
            fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
                self.0.as_ref().try_borrow_mut_data().map_err(Into::into)
            }
        }
    };
}

impl_writable_signer!(SystemAccount);
impl_writable_signer!(UncheckedAccount);

impl<T> WritableAccount for Mut<Signer<'_, Account<'_, T>>>
where
    T: Discriminator + RefFromBytes,
{
    type DataMut<'a>
        = RefMut<'a, [u8]>
    where
        Self: 'a;
    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        self.0.as_ref().try_borrow_mut_data().map_err(Into::into)
    }
}

impl<T> WritableAccount for Mut<Signer<'_, InterfaceAccount<'_, T>>>
where
    T: Discriminator + RefFromBytes,
{
    type DataMut<'a>
        = RefMut<'a, [u8]>
    where
        Self: 'a;
    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        self.0.as_ref().try_borrow_mut_data().map_err(Into::into)
    }
}

impl<T: Discriminator + RefFromBytes> WritableAccount for Mut<Account<'_, T>> {
    type DataMut<'a>
        = RefMut<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        RefMut::filter_map(self.0.as_ref().try_borrow_mut_data()?, T::read_mut)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<T: Discriminator + RefFromBytes> WritableAccount for Mut<InterfaceAccount<'_, T>> {
    type DataMut<'a>
        = RefMut<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        RefMut::filter_map(self.0.as_ref().try_borrow_mut_data()?, T::read_mut)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<T> SignerAccount for Mut<Signer<'_, T>> where T: ReadableAccount {}

#[doc(hidden)]
impl<'a, T> Mut<T>
where
    T: ReadableAccount + FromRaw<'a>,
{
    #[inline(always)]
    pub fn from_raw_info(info: &'a AccountInfo) -> Self {
        Mut(T::from_raw(info))
    }
}

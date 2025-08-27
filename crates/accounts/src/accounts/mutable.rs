use {
    super::{Account, Program, SystemAccount, UncheckedAccount},
    crate::{
        Discriminator, FromAccountInfo, FromRaw, ReadableAccount, RefFromBytes, Signer,
        SignerAccount, WritableAccount,
    },
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, RefMut},
        program_error::ProgramError,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub trait MutCheck {
    fn check(_info: &AccountInfo) -> Result<(), Error> {
        Ok(())
    }
}

pub struct Check;

impl MutCheck for Check {
    fn check(info: &AccountInfo) -> Result<(), Error> {
        if info.is_writable() {
            Ok(())
        } else {
            Err(ErrorCode::AccountNotMutable.into())
        }
    }
}

pub struct NoCheck;

impl MutCheck for NoCheck {}

pub struct Mut<T: ReadableAccount, C: MutCheck = Check> {
    pub(crate) acc: T,
    _phantom: PhantomData<C>,
}

impl<'a, T, C> FromAccountInfo<'a> for Mut<T, C>
where
    C: MutCheck,
    T: FromAccountInfo<'a> + ReadableAccount,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        C::check(info)?;

        Ok(Mut {
            acc: T::try_from_info(info)?,
            _phantom: PhantomData,
        })
    }
}

impl<T, C> AsRef<AccountInfo> for Mut<T, C>
where
    C: MutCheck,
    T: ReadableAccount,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.acc.as_ref()
    }
}

impl<'a, T, C> From<Mut<T, C>> for &'a AccountInfo
where
    C: MutCheck,
    T: ReadableAccount + Into<&'a AccountInfo>,
{
    #[inline(always)]
    fn from(value: Mut<T, C>) -> Self {
        value.acc.into()
    }
}

impl<T, C> ReadableAccount for Mut<T, C>
where
    C: MutCheck,
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

macro_rules! impl_writable {
    ($name: ident) => {
        impl<C: MutCheck> WritableAccount for Mut<$name<'_>, C> {
            type DataMut<'a>
                = RefMut<'a, [u8]>
            where
                Self: 'a;

            #[inline(always)]
            fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
                self.acc.as_ref().try_borrow_mut_data().map_err(Into::into)
            }
        }
    };
}

impl_writable!(Signer);
impl_writable!(SystemAccount);
impl_writable!(UncheckedAccount);

impl<T, C> WritableAccount for Mut<Program<'_, T>, C>
where
    C: MutCheck,
{
    type DataMut<'a>
        = RefMut<'a, [u8]>
    where
        Self: 'a;

    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        self.acc.as_ref().try_borrow_mut_data().map_err(Into::into)
    }
}

impl<T, C> WritableAccount for Mut<Account<'_, T>, C>
where
    C: MutCheck,
    T: Discriminator + RefFromBytes,
{
    type DataMut<'a>
        = RefMut<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        RefMut::filter_map(self.acc.as_ref().try_borrow_mut_data()?, T::read_mut)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<C: MutCheck> SignerAccount for Mut<Signer<'_>, C> {}

#[doc(hidden)]
impl<'a, T, C> Mut<T, C>
where
    C: MutCheck,
    T: ReadableAccount + FromRaw<'a>,
{
    #[inline(always)]
    pub fn from_raw_info(info: &'a AccountInfo) -> Self {
        Mut {
            acc: T::from_raw(info),
            _phantom: PhantomData,
        }
    }
}

use {
    super::Account,
    crate::{
        Discriminator, FromAccountInfo, FromRaw, InterfaceAccount, ReadableAccount, Signer,
        SignerAccount, SignerCheck, WritableAccount,
    },
    solana_account_view::{AccountView, RefMut},
    solana_program_error::ProgramError,
    typhoon_errors::Error,
    typhoon_traits::{AccountStrategy, MutAccessor},
};

pub struct Mut<T: ReadableAccount>(pub(crate) T);

impl<'a, T> FromAccountInfo<'a> for Mut<T>
where
    T: FromAccountInfo<'a> + ReadableAccount,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountView) -> Result<Self, Error> {
        Ok(Mut(T::try_from_info(info)?))
    }
}

impl<T> AsRef<AccountView> for Mut<T>
where
    T: ReadableAccount,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountView {
        self.0.as_ref()
    }
}

impl<'a, T> From<Mut<T>> for &'a AccountView
where
    T: ReadableAccount + Into<&'a AccountView>,
{
    #[inline(always)]
    fn from(value: Mut<T>) -> Self {
        value.0.into()
    }
}

impl<T> ReadableAccount for Mut<T> where T: ReadableAccount {}
impl<T> WritableAccount for Mut<T> where T: ReadableAccount {}

impl<T, C> Mut<Signer<'_, Account<'_, T>, C>>
where
    C: SignerCheck,
    T: Discriminator + AccountStrategy,
    <T as AccountStrategy>::Strategy: for<'a> MutAccessor<'a, T, Data = &'a mut T>,
{
    #[inline(always)]
    pub fn mut_data(&self) -> Result<RefMut<'_, T>, Error> {
        RefMut::try_map(self.0.info.try_borrow_mut()?, |data| {
            <<T as AccountStrategy>::Strategy as MutAccessor<'_, T>>::access_mut(
                &mut data[T::DISCRIMINATOR.len()..],
            )
        })
        .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<T, C> Mut<Signer<'_, InterfaceAccount<'_, T>, C>>
where
    C: SignerCheck,
    T: Discriminator + AccountStrategy,
    <T as AccountStrategy>::Strategy: for<'a> MutAccessor<'a, T, Data = &'a mut T>,
{
    #[inline(always)]
    pub fn mut_data(&self) -> Result<RefMut<'_, T>, Error> {
        RefMut::try_map(self.0.info.try_borrow_mut()?, |data| {
            <<T as AccountStrategy>::Strategy as MutAccessor<'_, T>>::access_mut(
                &mut data[T::DISCRIMINATOR.len()..],
            )
        })
        .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<T> Mut<Account<'_, T>>
where
    T: Discriminator + AccountStrategy,
    <T as AccountStrategy>::Strategy: for<'a> MutAccessor<'a, T, Data = &'a mut T>,
{
    #[inline(always)]
    pub fn mut_data(&self) -> Result<RefMut<'_, T>, Error> {
        RefMut::try_map(self.0.info.try_borrow_mut()?, |data| {
            <<T as AccountStrategy>::Strategy as MutAccessor<'_, T>>::access_mut(
                &mut data[T::DISCRIMINATOR.len()..],
            )
        })
        .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<T> Mut<InterfaceAccount<'_, T>>
where
    T: Discriminator + AccountStrategy,
    <T as AccountStrategy>::Strategy: for<'a> MutAccessor<'a, T, Data = &'a mut T>,
{
    #[inline(always)]
    pub fn mut_data(&self) -> Result<RefMut<'_, T>, Error> {
        RefMut::try_map(self.0.info.try_borrow_mut()?, |data| {
            <<T as AccountStrategy>::Strategy as MutAccessor<'_, T>>::access_mut(
                &mut data[T::DISCRIMINATOR.len()..],
            )
        })
        .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<T, C> SignerAccount for Mut<Signer<'_, T, C>>
where
    T: ReadableAccount,
    C: SignerCheck,
{
}

#[doc(hidden)]
impl<'a, T> Mut<T>
where
    T: ReadableAccount + FromRaw<'a>,
{
    #[inline(always)]
    pub fn from_raw_info(info: &'a AccountView) -> Self {
        Mut(T::from_raw(info))
    }
}

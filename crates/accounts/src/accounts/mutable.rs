use {
    super::{Account, Program, SystemAccount, UncheckedAccount},
    crate::{
        Discriminator, FromAccountInfo, ReadableAccount, RefFromBytes, Signer, SignerAccount,
        WritableAccount,
    },
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref, RefMut},
        program_error::ProgramError,
        pubkey::Pubkey,
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
    fn key(&self) -> &Pubkey {
        self.0.key()
    }

    #[inline(always)]
    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.0.is_owned_by(owner)
    }

    #[inline(always)]
    fn lamports(&self) -> Result<Ref<'_, u64>, Error> {
        self.0.lamports()
    }

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
            fn assign(&self, new_owner: &Pubkey) {
                unsafe {
                    self.0.as_ref().assign(new_owner);
                }
            }

            #[inline(always)]
            fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), Error> {
                self.0
                    .as_ref()
                    .realloc(new_len, zero_init)
                    .map_err(Into::into)
            }

            #[inline(always)]
            fn mut_lamports(&self) -> Result<RefMut<'_, u64>, Error> {
                self.0
                    .as_ref()
                    .try_borrow_mut_lamports()
                    .map_err(Into::into)
            }

            #[inline(always)]
            fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
                self.0.as_ref().try_borrow_mut_data().map_err(Into::into)
            }
        }
    };
}

impl_writable!(Signer);
impl_writable!(SystemAccount);
impl_writable!(UncheckedAccount);

impl<T> WritableAccount for Mut<Program<'_, T>> {
    type DataMut<'a>
        = RefMut<'a, [u8]>
    where
        Self: 'a;

    #[inline(always)]
    fn assign(&self, new_owner: &Pubkey) {
        unsafe {
            self.0.as_ref().assign(new_owner);
        }
    }

    #[inline(always)]
    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), Error> {
        self.0
            .as_ref()
            .realloc(new_len, zero_init)
            .map_err(Into::into)
    }

    #[inline(always)]
    fn mut_lamports(&self) -> Result<RefMut<'_, u64>, Error> {
        self.0
            .as_ref()
            .try_borrow_mut_lamports()
            .map_err(Into::into)
    }

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
    fn assign(&self, new_owner: &Pubkey) {
        unsafe {
            self.0.as_ref().assign(new_owner);
        }
    }

    #[inline(always)]
    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), Error> {
        self.0
            .as_ref()
            .realloc(new_len, zero_init)
            .map_err(Into::into)
    }

    #[inline(always)]
    fn mut_lamports(&self) -> Result<RefMut<'_, u64>, Error> {
        self.0
            .as_ref()
            .try_borrow_mut_lamports()
            .map_err(Into::into)
    }

    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        RefMut::filter_map(self.0.as_ref().try_borrow_mut_data()?, T::read_mut)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl SignerAccount for Mut<Signer<'_>> {}

impl<'a, T> Mut<Account<'a, T>>
where
    T: RefFromBytes + Discriminator,
{
    #[inline(always)]
    pub fn from_raw_info(info: &'a AccountInfo) -> Self {
        Mut(Account {
            info,
            _phantom: PhantomData,
        })
    }
}

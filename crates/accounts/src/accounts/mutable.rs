use {
    super::{Account, Program, SystemAccount, UncheckedAccount},
    crate::{
        Discriminator, FromAccountInfo, ReadableAccount, RefFromBytes, Signer, SignerAccount,
        WritableAccount,
    },
    pinocchio::{
        account_info::{AccountInfo, Ref, RefMut},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    typhoon_errors::Error,
};

pub struct Mut<T: ReadableAccount>(T);

impl<'a, T> FromAccountInfo<'a> for Mut<T>
where
    T: FromAccountInfo<'a> + ReadableAccount,
{
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, ProgramError> {
        if !info.is_writable() {
            return Err(Error::AccountNotMutable.into());
        }

        Ok(Mut(T::try_from_info(info)?))
    }
}

impl<T> AsRef<AccountInfo> for Mut<T>
where
    T: ReadableAccount,
{
    fn as_ref(&self) -> &AccountInfo {
        self.0.as_ref()
    }
}

impl<'a, T> From<Mut<T>> for &'a AccountInfo
where
    T: ReadableAccount + Into<&'a AccountInfo>,
{
    fn from(value: Mut<T>) -> Self {
        value.0.into()
    }
}

impl<T> ReadableAccount for Mut<T>
where
    T: ReadableAccount,
{
    type DataType = T::DataType;

    fn key(&self) -> &Pubkey {
        self.0.key()
    }

    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.0.is_owned_by(owner)
    }

    fn lamports(&self) -> Result<Ref<u64>, ProgramError> {
        self.0.lamports()
    }

    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError> {
        self.0.data()
    }
}

macro_rules! impl_writable {
    ($name: ident) => {
        impl WritableAccount for Mut<$name<'_>> {
            fn assign(&self, new_owner: &Pubkey) {
                unsafe {
                    self.0.as_ref().assign(new_owner);
                }
            }

            fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), ProgramError> {
                self.0.as_ref().realloc(new_len, zero_init)
            }

            fn mut_lamports(&self) -> Result<RefMut<u64>, ProgramError> {
                self.0.as_ref().try_borrow_mut_lamports()
            }

            fn mut_data(&self) -> Result<RefMut<Self::DataType>, ProgramError> {
                self.0.as_ref().try_borrow_mut_data()
            }
        }
    };
}

impl_writable!(Signer);
impl_writable!(SystemAccount);
impl_writable!(UncheckedAccount);

impl<T> WritableAccount for Mut<Program<'_, T>> {
    fn assign(&self, new_owner: &Pubkey) {
        unsafe {
            self.0.as_ref().assign(new_owner);
        }
    }

    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), ProgramError> {
        self.0.as_ref().realloc(new_len, zero_init)
    }

    fn mut_lamports(&self) -> Result<RefMut<u64>, ProgramError> {
        self.0.as_ref().try_borrow_mut_lamports()
    }

    fn mut_data(&self) -> Result<RefMut<Self::DataType>, ProgramError> {
        self.0.as_ref().try_borrow_mut_data()
    }
}

impl<T: Discriminator + RefFromBytes> WritableAccount for Mut<Account<'_, T>> {
    fn assign(&self, new_owner: &Pubkey) {
        unsafe {
            self.0.as_ref().assign(new_owner);
        }
    }

    fn realloc(&self, new_len: usize, zero_init: bool) -> Result<(), ProgramError> {
        self.0.as_ref().realloc(new_len, zero_init)
    }

    fn mut_lamports(&self) -> Result<RefMut<u64>, ProgramError> {
        self.0.as_ref().try_borrow_mut_lamports()
    }

    fn mut_data(&self) -> Result<RefMut<Self::DataType>, ProgramError> {
        RefMut::filter_map(self.0.as_ref().try_borrow_mut_data()?, T::read_mut)
            .map_err(|_| ProgramError::InvalidAccountData)
    }
}

impl SignerAccount for Mut<Signer<'_>> {}

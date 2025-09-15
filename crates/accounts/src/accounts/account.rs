use {
    crate::{
        discriminator_matches, Discriminator, FromAccountInfo, FromRaw, Mut, Owner,
        ReadableAccount, RefFromBytes,
    },
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        hint::unlikely,
        program_error::ProgramError,
        pubkey::pubkey_eq,
    },
    typhoon_errors::{Error, ErrorCode},
};

pub struct Account<'a, T>
where
    T: Discriminator + RefFromBytes,
{
    pub(crate) info: &'a AccountInfo,
    pub(crate) _phantom: PhantomData<T>,
}

impl<'a, T> Account<'a, T>
where
    T: Discriminator + RefFromBytes,
{
    pub fn data_unchecked(&self) -> Result<&T, Error> {
        let dis_len = T::DISCRIMINATOR.len();
        let total_len = dis_len + core::mem::size_of::<T>();

        if self.info.data_len() < total_len {
            return Err(ProgramError::InvalidAccountData.into());
        }

        let data_ptr = unsafe { self.info.data_ptr().add(dis_len) };

        if data_ptr.align_offset(core::mem::align_of::<T>()) != 0 {
            return Err(ProgramError::InvalidAccountData.into());
        }

        Ok(unsafe { &*(data_ptr as *const T) })
    }
}

impl<'a, T> Mut<Account<'a, T>>
where
    T: Discriminator + RefFromBytes,
{
    pub fn data_unchecked(&self) -> Result<&T, Error> {
        self.0.data_unchecked()
    }
}

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: Owner + Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        let account_data = info.try_borrow_data()?;

        // Check data length first - this is the cheapest check and most likely to fail
        if unlikely(account_data.len() < T::DISCRIMINATOR.len()) {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        // Validate discriminator using optimized comparison for small discriminators
        if unlikely(!discriminator_matches::<T>(&account_data)) {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }

        // Verify account ownership - checked after discriminator for better branch prediction
        if unlikely(!pubkey_eq(info.owner(), &T::OWNER)) {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        // Handle special case: zero-lamport system accounts (least common case)
        if unlikely(pubkey_eq(info.owner(), &pinocchio_system::ID)) {
            // Only perform additional lamports check for system accounts
            if *info.try_borrow_lamports()? == 0 {
                return Err(ProgramError::UninitializedAccount.into());
            }
        }

        Ok(Account {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Account<'a, T>> for &'a AccountInfo
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn from(value: Account<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for Account<'_, T>
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for Account<'_, T>
where
    T: RefFromBytes + Discriminator,
{
    type Data<'a>
        = Ref<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ref::filter_map(self.info.try_borrow_data()?, T::read)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

impl<'a, T> FromRaw<'a> for Account<'a, T>
where
    T: RefFromBytes + Discriminator,
{
    fn from_raw(info: &'a AccountInfo) -> Self {
        Self {
            info,
            _phantom: PhantomData,
        }
    }
}

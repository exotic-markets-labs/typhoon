use {
    crate::{
        discriminator_matches, FromAccountInfo, FromRaw, ReadableAccount, RefFromBytes, System,
    },
    core::marker::PhantomData,
    pinocchio::hint::unlikely,
    solana_account_view::{AccountView, Ref},
    solana_program_error::ProgramError,
    typhoon_errors::{Error, ErrorCode},
    typhoon_traits::{CheckOwner, CheckProgramId, Discriminator},
};

pub struct Account<'a, T>
where
    T: Discriminator + RefFromBytes,
{
    pub(crate) info: &'a AccountView,
    pub(crate) _phantom: PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: CheckOwner + Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountView) -> Result<Self, Error> {
        // Check data length first - this is the cheapest check and most likely to fail
        if unlikely(info.data_len() < T::DISCRIMINATOR.len()) {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        // Validate discriminator using optimized comparison for small discriminators
        if unlikely(!discriminator_matches::<T>(info)) {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }

        let owner = unsafe { info.owner() };

        // Verify account ownership - checked after discriminator for better branch prediction
        if unlikely(!T::owned_by(owner)) {
            return Err(ProgramError::InvalidAccountOwner.into());
        }

        // Handle special case: zero-lamport system accounts (least common case)
        if unlikely(System::address_eq(owner)) {
            // Only perform additional lamports check for system accounts
            if info.lamports() == 0 {
                return Err(ProgramError::UninitializedAccount.into());
            }
        }

        Ok(Account {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> From<Account<'a, T>> for &'a AccountView
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn from(value: Account<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountView> for Account<'_, T>
where
    T: Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountView {
        self.info
    }
}

impl<T> ReadableAccount for Account<'_, T>
where
    T: RefFromBytes + Discriminator,
{
    type DataUnchecked = T;
    type Data<'a>
        = Ref<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ref::filter_map(self.info.try_borrow()?, T::read)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }

    #[inline]
    fn data_unchecked(&self) -> Result<&Self::DataUnchecked, Error> {
        let dis_len = T::DISCRIMINATOR.len();
        let total_len = dis_len + core::mem::size_of::<T>();

        if self.info.data_len() < total_len {
            return Err(ErrorCode::InvalidDataLength.into());
        }

        let data_ptr = unsafe { self.info.data_ptr().add(dis_len) };

        if data_ptr.align_offset(core::mem::align_of::<T>()) != 0 {
            return Err(ErrorCode::InvalidDataAlignment.into());
        }

        Ok(unsafe { &*(data_ptr as *const T) })
    }
}

impl<'a, T> FromRaw<'a> for Account<'a, T>
where
    T: RefFromBytes + Discriminator,
{
    fn from_raw(info: &'a AccountView) -> Self {
        Self {
            info,
            _phantom: PhantomData,
        }
    }
}

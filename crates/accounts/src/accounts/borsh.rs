use {
    super::Mut,
    crate::{Discriminator, FromAccountInfo, Owner, ReadableAccount, WritableAccount},
    core::cell::RefCell,
    pinocchio::{account_info::AccountInfo, program_error::ProgramError},
    typhoon_errors::{Error, ErrorCode},
};

pub struct BorshAccount<'a, T>
where
    T: Discriminator,
{
    info: &'a AccountInfo,
    data: RefCell<T>,
}

impl<'a, T> FromAccountInfo<'a> for BorshAccount<'a, T>
where
    T: Owner + Discriminator + borsh::BorshSerialize + borsh::BorshDeserialize,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        // Validate account and deserialize in a single pass
        let state = Self::validate_and_deserialize_fast_path(info)?;

        Ok(BorshAccount {
            info,
            data: RefCell::new(state),
        })
    }
}

impl<'a, T> BorshAccount<'a, T>
where
    T: Owner + Discriminator + borsh::BorshSerialize + borsh::BorshDeserialize,
{
    /// Fast-path borsh account validation and deserialization with optimized branch prediction.
    #[inline(always)]
    fn validate_and_deserialize_fast_path(info: &AccountInfo) -> Result<T, Error> {
        // Borrow account data once for all validation checks and deserialization
        let account_data = info.try_borrow_data()?;

        // Check data length first - this is the cheapest check and most likely to fail
        if unlikely(account_data.len() < T::DISCRIMINATOR.len()) {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        // Split data once for validation and deserialization
        let (discriminator, mut data) = account_data.split_at(T::DISCRIMINATOR.len());

        // Validate discriminator using optimized comparison for small discriminators
        if unlikely(!discriminator_matches_slice::<T>(discriminator)) {
            return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        }

        // Verify account ownership - checked after discriminator for better branch prediction
        if unlikely(!info.is_owned_by(&T::OWNER)) {
            return Err(ErrorCode::AccountOwnedByWrongProgram.into());
        }

        // Handle special case: zero-lamport system accounts (least common case)
        if unlikely(info.is_owned_by(&pinocchio_system::ID)) {
            // Only perform additional lamports check for system accounts
            if *info.try_borrow_lamports()? == 0 {
                return Err(ProgramError::UninitializedAccount.into());
            }
        }

        // Deserialize the state data (this is the most expensive operation, done last)
        let state = T::deserialize(&mut data).map_err(|_| ProgramError::BorshIoError)?;

        Ok(state)
    }
}

/// Optimized discriminator matching for pre-split discriminator slice.
///
/// Similar to the main discriminator_matches function but optimized for when we already
/// have the discriminator slice separated (as is the case in BorshAccount deserialization).
#[inline(always)]
fn discriminator_matches_slice<T: Discriminator>(discriminator_slice: &[u8]) -> bool {
    let expected = T::DISCRIMINATOR;
    let len = expected.len();

    // Choose optimal comparison strategy based on discriminator length
    match len {
        0 => true, // No discriminator to check
        1..=8 => {
            // Use unaligned integer reads for small discriminators (most common case)
            // SAFETY: We've already verified that discriminator_slice.len() >= expected.len()
            // through the split_at call in the caller, so we know we have exactly `len` bytes.
            unsafe {
                let data_ptr = discriminator_slice.as_ptr() as *const u64;
                let disc_ptr = expected.as_ptr() as *const u64;

                match len {
                    1 => *discriminator_slice.get_unchecked(0) == *expected.get_unchecked(0),
                    2 => {
                        let data_val = (data_ptr as *const u16).read_unaligned();
                        let disc_val = (disc_ptr as *const u16).read_unaligned();
                        data_val == disc_val
                    }
                    4 => {
                        let data_val = (data_ptr as *const u32).read_unaligned();
                        let disc_val = (disc_ptr as *const u32).read_unaligned();
                        data_val == disc_val
                    }
                    8 => {
                        let data_val = data_ptr.read_unaligned();
                        let disc_val = disc_ptr.read_unaligned();
                        data_val == disc_val
                    }
                    _ => discriminator_slice == expected,
                }
            }
        }
        9..=16 => {
            // IMPORTANT: SIMD comparison ONLY triggers for OFF-CHAIN native execution
            // On-chain BPF programs will always use the standard slice comparison fallback
            simd_compare_discriminator_slice(discriminator_slice, expected, len)
        }
        _ => {
            // Standard slice comparison for large discriminators
            discriminator_slice == expected
        }
    }
}

/// SIMD-optimized discriminator comparison for pre-split slices.
#[inline(always)]
fn simd_compare_discriminator_slice(data: &[u8], discriminator: &[u8], len: usize) -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        simd_compare_x86_64_slice(data, discriminator, len)
    }
    #[cfg(target_arch = "aarch64")]
    {
        simd_compare_aarch64_slice(data, discriminator, len)
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        // Safe fallback for all other architectures
        data == discriminator
    }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn simd_compare_x86_64_slice(data: &[u8], discriminator: &[u8], len: usize) -> bool {
    if len == 16 {
        unsafe {
            use core::arch::x86_64::*;
            let data_vec = _mm_loadu_si128(data.as_ptr() as *const __m128i);
            let disc_vec = _mm_loadu_si128(discriminator.as_ptr() as *const __m128i);
            let cmp = _mm_cmpeq_epi8(data_vec, disc_vec);
            _mm_movemask_epi8(cmp) == 0xFFFF
        }
    } else {
        data == discriminator
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn simd_compare_aarch64_slice(data: &[u8], discriminator: &[u8], len: usize) -> bool {
    if len == 16 {
        unsafe {
            use core::arch::aarch64::*;
            let data_vec = vld1q_u8(data.as_ptr());
            let disc_vec = vld1q_u8(discriminator.as_ptr());
            let cmp = vceqq_u8(data_vec, disc_vec);
            let min_val = vminvq_u8(cmp);
            min_val == 0xFF
        }
    } else {
        data == discriminator
    }
}

/// Cold function used for branch prediction hints in stable Rust.
#[inline(always)]
#[cold]
fn cold() {}

/// Branch prediction hint for unlikely conditions.
#[inline(always)]
fn unlikely(b: bool) -> bool {
    if b {
        cold();
    }
    b
}

impl<'a, T> From<BorshAccount<'a, T>> for &'a AccountInfo
where
    T: Owner + Discriminator,
{
    #[inline(always)]
    fn from(value: BorshAccount<'a, T>) -> Self {
        value.info
    }
}

impl<T> AsRef<AccountInfo> for BorshAccount<'_, T>
where
    T: Discriminator,
{
    #[inline(always)]
    fn as_ref(&self) -> &AccountInfo {
        self.info
    }
}

impl<T> ReadableAccount for BorshAccount<'_, T>
where
    T: Discriminator,
{
    type Data<'a>
        = core::cell::Ref<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ok(self.data.borrow())
    }
}

impl<T> WritableAccount for Mut<BorshAccount<'_, T>>
where
    T: Discriminator,
{
    type DataMut<'a>
        = core::cell::RefMut<'a, T>
    where
        Self: 'a;

    #[inline(always)]
    fn mut_data<'a>(&'a self) -> Result<Self::DataMut<'a>, Error> {
        self.0
            .data
            .try_borrow_mut()
            .map_err(|_| ProgramError::AccountBorrowFailed.into())
    }
}

impl<T> Mut<BorshAccount<'_, T>>
where
    T: Discriminator + borsh::BorshSerialize,
{
    #[inline(always)]
    pub fn serialize(&self) -> Result<(), Error> {
        let data = self
            .0
            .data
            .try_borrow()
            .map_err(|_| ProgramError::AccountBorrowFailed)?;

        data.serialize(&mut self.0.info.try_borrow_mut_data()?.as_mut())
            .map_err(|_| ProgramError::BorshIoError.into())
    }
}

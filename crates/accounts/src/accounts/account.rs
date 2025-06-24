use {
    crate::{Discriminator, FromAccountInfo, Owner, ReadableAccount, RefFromBytes},
    core::marker::PhantomData,
    pinocchio::{
        account_info::{AccountInfo, Ref},
        program_error::ProgramError,
        pubkey::Pubkey,
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

impl<'a, T> FromAccountInfo<'a> for Account<'a, T>
where
    T: Owner + Discriminator + RefFromBytes,
{
    #[inline(always)]
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        // Validate account ownership, discriminator, and data integrity in a single pass
        Self::validate_account_fast_path(info)?;

        Ok(Account {
            info,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> Account<'a, T>
where
    T: Owner + Discriminator + RefFromBytes,
{
    /// Fast-path account validation with reduced syscalls and optimized branch prediction.
    ///
    /// This method combines multiple validation steps to minimize runtime overhead:
    /// - Single data borrow instead of separate lamports/data borrows
    /// - Ordered checks from most-likely-to-fail to least-likely-to-fail
    /// - Branch prediction hints for common failure cases
    #[inline(always)]
    fn validate_account_fast_path(info: &AccountInfo) -> Result<(), Error> {
        // Borrow account data once for all validation checks
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

        Ok(())
    }
}

/// discriminator matching with length-optimized comparison strategies.
///
/// Uses different comparison methods based on discriminator length:
/// - 1-8 bytes: Unaligned integer reads for maximum performance
/// - 9-16 bytes: SIMD comparison on supported architectures
/// - >16 bytes: Standard slice comparison
#[inline(always)]
fn discriminator_matches<T: Discriminator>(data: &[u8]) -> bool {
    const MAX_SIMD_DISCRIMINATOR: usize = 16;

    let discriminator = T::DISCRIMINATOR;
    let len = discriminator.len();

    // Choose optimal comparison strategy based on discriminator length
    match len {
        0 => true, // No discriminator to check
        1..=8 => {
            // Use unaligned integer reads for small discriminators (most common case)
            // SAFETY: We've already verified that data.len() >= discriminator.len() 
            // in the caller before calling this function, so we know we have at least 
            // `len` bytes available for reading. Unaligned reads are safe for primitive 
            // types on all supported architectures. The pointer casts to smaller integer 
            // types (u16, u32, u64) are valid because we're only reading the exact number 
            // of bytes specified by `len`.
            unsafe {
                let data_ptr = data.as_ptr() as *const u64;
                let disc_ptr = discriminator.as_ptr() as *const u64;

                match len {
                    1 => *data.get_unchecked(0) == *discriminator.get_unchecked(0),
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
                    _ => data[..len] == discriminator[..],
                }
            }
        }
        9..=MAX_SIMD_DISCRIMINATOR => {
            // Use SIMD comparison for medium-sized discriminators on supported architectures
            simd_compare_discriminator(data, discriminator, len)
        }
        _ => {
            // Standard slice comparison for large discriminators
            data[..len] == discriminator[..]
        }
    }
}

/// SIMD-optimized discriminator comparison with multi-architecture support.
///
/// Provides optimized implementations for:
/// - x86_64: SSE2 instructions for 16-byte comparisons
/// - aarch64: NEON instructions for 16-byte comparisons  
/// - Other architectures: Safe fallback to slice comparison
#[inline(always)]
fn simd_compare_discriminator(data: &[u8], discriminator: &[u8], len: usize) -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        simd_compare_x86_64(data, discriminator, len)
    }
    #[cfg(target_arch = "aarch64")]
    {
        simd_compare_aarch64(data, discriminator, len)
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        // Safe fallback for all other architectures
        data[..len] == discriminator[..]
    }
}

/// x86_64 SIMD implementation using SSE2 instructions.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn simd_compare_x86_64(data: &[u8], discriminator: &[u8], len: usize) -> bool {
    if len == 16 {
        // SAFETY: We've verified len == 16, so both data and discriminator have at least
        // 16 bytes. The _mm_loadu_si128 instruction can safely read 16 bytes from both
        // pointers. SSE2 is available on all x86_64 processors.
        unsafe {
            use core::arch::x86_64::*;
            // Use SSE2 for 16-byte comparison
            let data_vec = _mm_loadu_si128(data.as_ptr() as *const __m128i);
            let disc_vec = _mm_loadu_si128(discriminator.as_ptr() as *const __m128i);
            let cmp = _mm_cmpeq_epi8(data_vec, disc_vec);
            _mm_movemask_epi8(cmp) == 0xFFFF
        }
    } else {
        // Fallback for non-16-byte lengths
        data[..len] == discriminator[..]
    }
}

/// ARM64 SIMD implementation using NEON instructions.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn simd_compare_aarch64(data: &[u8], discriminator: &[u8], len: usize) -> bool {
    if len == 16 {
        // SAFETY: We've verified len == 16, so both data and discriminator have at least
        // 16 bytes. The vld1q_u8 instruction can safely load 16 bytes from both pointers.
        // NEON is available on all aarch64 processors.
        unsafe {
            use core::arch::aarch64::*;
            // Use NEON for 16-byte comparison
            let data_vec = vld1q_u8(data.as_ptr());
            let disc_vec = vld1q_u8(discriminator.as_ptr());
            let cmp = vceqq_u8(data_vec, disc_vec);
            // Check if all bytes are equal
            let min_val = vminvq_u8(cmp);
            min_val == 0xFF
        }
    } else {
        // Fallback for non-16-byte lengths
        data[..len] == discriminator[..]
    }
}

/// Cold function used for branch prediction hints in stable Rust.
///
/// This function is marked as `#[cold]` to hint to the compiler that any
/// branch containing a call to this function is unlikely to be taken.
#[inline(always)]
#[cold]
fn cold() {}

/// Branch prediction hint for unlikely conditions.
///
/// Uses the stable `#[cold]` function approach to provide branch prediction hints.
/// This works by calling a cold function in the unlikely branch, which signals
/// to LLVM that this branch should be optimized for the uncommon case.
#[inline(always)]
fn unlikely(b: bool) -> bool {
    if b {
        cold();
    }
    b
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
    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    #[inline(always)]
    fn is_owned_by(&self, owner: &Pubkey) -> bool {
        self.info.is_owned_by(owner)
    }

    #[inline(always)]
    fn lamports(&self) -> Result<Ref<'_, u64>, Error> {
        self.info.try_borrow_lamports().map_err(Into::into)
    }

    #[inline(always)]
    fn data<'a>(&'a self) -> Result<Self::Data<'a>, Error> {
        Ref::filter_map(self.info.try_borrow_data()?, T::read)
            .map_err(|_| ProgramError::InvalidAccountData.into())
    }
}

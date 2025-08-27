use crate::Discriminator;

/// discriminator matching with length-optimized comparison strategies.
///
/// **EXECUTION CONTEXT IMPORTANT:**
/// - **On-chain (BPF/SVM)**: Always uses standard slice comparison fallbacks
/// - **Off-chain (Native)**: Uses SIMD optimizations when available for RPC nodes, indexers, wallets, etc.
///
/// Uses different comparison methods based on discriminator length:
/// - 1-8 bytes: Unaligned integer reads for maximum performance
/// - 9-16 bytes: SIMD comparison on supported architectures (OFF-CHAIN ONLY)
/// - >16 bytes: Standard slice comparison
#[inline(always)]
pub fn discriminator_matches<T: Discriminator>(data: &[u8]) -> bool {
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
            // IMPORTANT: SIMD comparison ONLY triggers for OFF-CHAIN native execution
            // On-chain BPF programs will always use the standard slice comparison fallback
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
/// **CRITICAL: OFF-CHAIN EXECUTION ONLY**
/// This function provides SIMD optimizations that are ONLY available during off-chain execution:
///
/// **ON-CHAIN BEHAVIOR:**
/// When compiled for BPF targets, the conditional compilation ensures that on-chain
/// programs automatically fall back to standard slice comparison. The SVM does not
/// expose native SIMD instructions like SSE2 or NEON to BPF programs.
///
/// Provides optimized implementations for:
/// - x86_64: SSE2 instructions for 16-byte comparisons (OFF-CHAIN ONLY)
/// - aarch64: NEON instructions for 16-byte comparisons (OFF-CHAIN ONLY)
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
///
/// **OFF-CHAIN NATIVE EXECUTION ONLY**
/// This function uses SSE2 SIMD instructions that are only available when running
/// natively on x86_64 processors. BPF programs compiled for on-chain execution
/// will never call this function due to conditional compilation.
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
///
/// **OFF-CHAIN NATIVE EXECUTION ONLY**
/// This function uses NEON SIMD instructions that are only available when running
/// natively on aarch64 processors. BPF programs compiled for on-chain execution
/// will never call this function due to conditional compilation.
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

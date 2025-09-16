use {crate::Discriminator, pinocchio::account_info::AccountInfo};

/// Discriminator matching with length-optimized comparison strategies.
/// Uses different comparison methods based on discriminator length:
/// - `1-8` bytes: Unaligned integer reads for maximum performance
/// - `>8` bytes: Standard slice comparison
#[inline(always)]
pub fn discriminator_matches<T: Discriminator>(info: &AccountInfo) -> bool {
    let discriminator = T::DISCRIMINATOR;
    let len = discriminator.len();
    let data_ptr = info.data_ptr();

    // Choose optimal comparison strategy based on discriminator length

    // Use unaligned integer reads for small discriminators (most common case)
    // SAFETY: We've already verified that data.len() >= discriminator.len()
    // in the caller before calling this function, so we know we have at least
    // `len` bytes available for reading. Unaligned reads are safe for primitive
    // types on all supported architectures. The pointer casts to smaller integer
    // types (u16, u32, u64) are valid because we're only reading the exact number
    // of bytes specified by `len`.
    unsafe {
        match len {
            0 => true, // No discriminator to check
            1 => *data_ptr == discriminator[0],
            2 => {
                let data_val = (data_ptr as *const u16).read_unaligned();
                let disc_val = u16::from_le_bytes([discriminator[0], discriminator[1]]);
                data_val == disc_val
            }
            4 => {
                let data_val = (data_ptr as *const u32).read_unaligned();
                let disc_val = u32::from_le_bytes([
                    discriminator[0],
                    discriminator[1],
                    discriminator[2],
                    discriminator[3],
                ]);
                data_val == disc_val
            }
            8 => {
                let data_val = (data_ptr as *const u64).read_unaligned();
                let disc_val = u64::from_le_bytes([
                    discriminator[0],
                    discriminator[1],
                    discriminator[2],
                    discriminator[3],
                    discriminator[4],
                    discriminator[5],
                    discriminator[6],
                    discriminator[7],
                ]);
                data_val == disc_val
            }
            _ => {
                let data = core::slice::from_raw_parts(data_ptr, len);
                data == discriminator
            }
        }
    }
}

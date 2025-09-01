use crate::Discriminator;

/// Discriminator matching with length-optimized comparison strategies.
/// Uses different comparison methods based on discriminator length:
/// - 1-8 bytes: Unaligned integer reads for maximum performance
/// - >8 bytes: Standard slice comparison
#[inline(always)]
pub fn discriminator_matches<T: Discriminator>(data: &[u8]) -> bool {
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
        _ => data[..len] == discriminator[..],
    }
}

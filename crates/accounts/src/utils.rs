#[inline]
pub fn fast_32_byte_eq(a: &[u8], b: &[u8]) -> bool {
    unsafe {
        let a_ptr = a.as_ptr() as *const u64;
        let b_ptr = b.as_ptr() as *const u64;

        core::ptr::read_unaligned(a_ptr) == core::ptr::read_unaligned(b_ptr)
            && core::ptr::read_unaligned(a_ptr.add(1)) == core::ptr::read_unaligned(b_ptr.add(1))
            && core::ptr::read_unaligned(a_ptr.add(2)) == core::ptr::read_unaligned(b_ptr.add(2))
            && core::ptr::read_unaligned(a_ptr.add(3)) == core::ptr::read_unaligned(b_ptr.add(3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_32_byte_eq_identical_arrays() {
        let a = [1u8; 32];
        let b = [1u8; 32];
        assert!(fast_32_byte_eq(&a, &b));
    }

    #[test]
    fn test_fast_32_byte_eq_different_arrays() {
        let a = [1u8; 32];
        let mut b = [1u8; 32];
        b[31] = 2;
        assert!(!fast_32_byte_eq(&a, &b));
    }

    #[test]
    fn test_fast_32_byte_eq_all_zeros() {
        let a = [0u8; 32];
        let b = [0u8; 32];
        assert!(fast_32_byte_eq(&a, &b));
    }

    #[test]
    fn test_fast_32_byte_eq_all_max() {
        let a = [255u8; 32];
        let b = [255u8; 32];
        assert!(fast_32_byte_eq(&a, &b));
    }

    #[test]
    fn test_fast_32_byte_eq_difference_in_middle() {
        let mut a = [0u8; 32];
        let mut b = [0u8; 32];
        a[16] = 1;
        b[16] = 2;
        assert!(!fast_32_byte_eq(&a, &b));
    }

    #[test]
    fn test_fast_32_byte_eq_random_pattern() {
        let a = [
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77, 0x88, 0x99,
        ];
        let b = a.clone();
        assert!(fast_32_byte_eq(&a, &b));
    }

    #[test]
    fn test_fast_32_byte_eq_one_bit_difference() {
        let mut a = [0u8; 32];
        let mut b = [0u8; 32];
        a[0] = 0b00000001;
        b[0] = 0b00000010;
        assert!(!fast_32_byte_eq(&a, &b));
    }
}

use {crate::BorshAccessor, pinocchio::pubkey::Pubkey};

pub trait BorshWriter {
    fn write(data: &mut [u8], el: Self);
}

impl BorshWriter for &str {
    fn write(data: &mut [u8], el: Self) {
        let old_len = u32::convert(&data[..4]) as usize;
        let new_len = el.len();

        if 4 + new_len < data.len() {
            data.copy_within((4 + old_len).., 4 + new_len);
        }
        u32::write(data, new_len as u32);
        data[4..(new_len + 4)].copy_from_slice(el.as_bytes());
    }
}

impl BorshWriter for &Pubkey {
    fn write(data: &mut [u8], el: Self) {
        data.copy_from_slice(el);
    }
}

macro_rules! impl_num_writer {
    ($ty:ident, $len:literal) => {
        impl BorshWriter for $ty {
            fn write(data: &mut [u8], el: Self) {
                data[..$len].copy_from_slice(&el.to_le_bytes());
            }
        }
    };
}

impl_num_writer!(i8, 1);
impl_num_writer!(u8, 1);
impl_num_writer!(i16, 2);
impl_num_writer!(u16, 2);
impl_num_writer!(i32, 4);
impl_num_writer!(u32, 4);
impl_num_writer!(i64, 8);
impl_num_writer!(u64, 8);
impl_num_writer!(i128, 16);
impl_num_writer!(u128, 16);

#[cfg(test)]
mod tests {
    use {
        super::*,
        borsh::{BorshDeserialize, BorshSerialize},
    };

    #[test]
    fn test_str_writer() {
        let s = "hello";
        let mut buf = [0u8; 9];
        <&str as BorshWriter>::write(&mut buf, s);

        let len = u32::convert(&buf[..4]);
        assert_eq!(len, 5);
        assert_eq!(&buf[4..9], b"hello");
    }

    #[test]
    fn test_str_with_other() {
        let mut buf = [0u8; 14];
        "hello".serialize(&mut buf.as_mut_slice()).unwrap();
        10.serialize(&mut buf.as_mut_slice()).unwrap();

        <&str as BorshWriter>::write(&mut buf, "hello2");
        let result = String::deserialize(&mut buf.as_slice()).unwrap();
        assert_eq!(result, "hello2");
    }
}

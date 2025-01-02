use {
    std::{
        fmt::{Debug, Display},
        ops::Deref,
    },
    zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout},
};

#[repr(C)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct ZCStr<const MAX_SIZE: usize> {
    /// The bytes of the string.
    value: [u8; MAX_SIZE],
}

impl<const MAX_SIZE: usize> ZCStr<MAX_SIZE> {
    pub fn copy_from_slice(&mut self, slice: &[u8]) {
        let length = std::cmp::min(slice.len(), MAX_SIZE);
        self.value[..length].clone_from_slice(&slice[..length]);
        self.value[length..].fill(0);
    }

    /// Copy the content of a `&str` into the pod str.
    pub fn copy_from_str(&mut self, string: &str) {
        self.copy_from_slice(string.as_bytes())
    }
}
impl<const MAX_SIZE: usize> Default for ZCStr<MAX_SIZE> {
    fn default() -> Self {
        Self {
            value: [0; MAX_SIZE],
        }
    }
}

impl<const MAX_SIZE: usize> Deref for ZCStr<MAX_SIZE> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let data = unsafe { std::slice::from_raw_parts(self.value.as_ptr() as _, MAX_SIZE) };
        let end_index = data.iter().position(|&x| x == b'\0').unwrap_or(MAX_SIZE);

        unsafe { std::str::from_utf8_unchecked(&data[..end_index]) }
    }
}

impl<const MAX_SIZE: usize> Display for ZCStr<MAX_SIZE> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self)
    }
}

impl<const MAX_SIZE: usize> Debug for ZCStr<MAX_SIZE> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self)
    }
}

impl<const MAX_SIZE: usize> From<&str> for ZCStr<MAX_SIZE> {
    fn from(s: &str) -> Self {
        let mut value = ZCStr::default();
        value.copy_from_str(s);
        value
    }
}

impl<const MAX_SIZE: usize> From<String> for ZCStr<MAX_SIZE> {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl<const MAX_SIZE: usize> PartialEq<str> for ZCStr<MAX_SIZE> {
    fn eq(&self, other: &str) -> bool {
        self.deref() == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let str = ZCStr::<10>::from("str");
        assert_eq!(&str, "str");
    }

    #[test]
    fn test_copy_from_slice() {
        let mut str = ZCStr::<10>::from("empty");
        assert_eq!(&str, "empty");

        // Copy a slice that is equal to the max size.
        str.copy_from_str("emptyempty");
        assert_eq!(&str, "emptyempty");

        // Copy a slice that is smaller than the max size.
        str.copy_from_str("empty");
        assert_eq!(&str, "empty");

        // Copy a slice that is bigger than the max size.
        str.copy_from_str("emptyemptyempty");
        assert_eq!(&str, "emptyempty");
    }
}

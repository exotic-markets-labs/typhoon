use pinocchio::pubkey::Pubkey;

/// Defines the space of an account for initialization.
pub trait BorshSize {
    const SIZE: usize;
}

macro_rules! impl_size {
    ($ty:ident => $space:literal) => {
        impl BorshSize for $ty {
            const SIZE: usize = $space;
        }
    };
    (($($ty:ident),+) => $space:literal) => {
        $(
            impl_size!($ty => $space);
        )+

    };
}

impl_size!((i8, u8, bool) => 1);
impl_size!((i16, u16) => 2);
impl_size!((i32, u32, f32) => 4);
impl_size!((i64, u64, f64) => 8);
impl_size!((i128, u128) => 16);
impl_size!(Pubkey => 32);

impl<T: BorshSize> BorshSize for Option<T> {
    const SIZE: usize = 1 + T::SIZE;
}

pub const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

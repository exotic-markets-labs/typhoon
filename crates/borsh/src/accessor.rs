use pinocchio::pubkey::Pubkey;

pub trait BorshAccessor<'a>
where
    Self: 'a,
{
    fn len(data: &'a [u8]) -> usize;
    fn convert(data: &'a [u8]) -> Self;
}

impl<'a> BorshAccessor<'a> for &'a str {
    #[inline(always)]
    fn len(data: &'a [u8]) -> usize {
        4 + u32::from_le_bytes(data[..4].try_into().unwrap()) as usize
    }

    #[inline(always)]
    fn convert(data: &'a [u8]) -> Self {
        unsafe { core::str::from_utf8_unchecked(&data[4..]) }
    }
}

macro_rules! impl_num_accessor {
    ($ty:ident, $index:literal) => {
        impl BorshAccessor<'_> for $ty {
            #[inline(always)]
            fn len(_: &'_ [u8]) -> usize {
                $index
            }

            #[inline(always)]
            fn convert(data: &'_ [u8]) -> Self {
                $ty::from_le_bytes(data[..$index].try_into().unwrap())
            }
        }
    };
}

impl_num_accessor!(i8, 1);
impl_num_accessor!(u8, 1);
impl_num_accessor!(i16, 2);
impl_num_accessor!(u16, 2);
impl_num_accessor!(i32, 4);
impl_num_accessor!(u32, 4);
impl_num_accessor!(i64, 8);
impl_num_accessor!(u64, 8);
impl_num_accessor!(i128, 16);
impl_num_accessor!(u128, 16);

impl<'a> BorshAccessor<'a> for &'a Pubkey {
    #[inline(always)]
    fn len(_: &'a [u8]) -> usize {
        32
    }

    #[inline(always)]
    fn convert(data: &'a [u8]) -> Self {
        data[..32].try_into().unwrap()
    }
}

impl BorshAccessor<'_> for bool {
    #[inline(always)]
    fn len(_: &'_ [u8]) -> usize {
        1
    }

    #[inline(always)]
    fn convert(data: &'_ [u8]) -> Self {
        data[0] != 0
    }
}

use {
    crate::Discriminator,
    zerocopy::{FromBytes, IntoBytes, KnownLayout},
};

pub trait ReadMut {
    fn read_mut(data: &mut [u8]) -> Option<&mut Self>;
}

impl ReadMut for [u8] {
    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        Some(data)
    }
}

impl<T> ReadMut for T
where
    T: IntoBytes + KnownLayout + FromBytes + Discriminator,
{
    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        let dis_len = T::DISCRIMINATOR.len();
        T::mut_from_bytes(&mut data[dis_len..std::mem::size_of::<Self>() + dis_len]).ok()
    }
}

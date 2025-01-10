use {
    bytemuck::Pod,
    typhoon_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo},
};

pub trait ProgramId {
    const ID: Pubkey;
}

pub trait Owner {
    const OWNER: Pubkey;
}

pub trait Discriminator {
    const DISCRIMINATOR: &'static [u8];
}

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError>;
}

pub trait RefFromBytes {
    fn read(data: &[u8]) -> Option<&Self>;
    fn read_mut(data: &mut [u8]) -> Option<&mut Self>;
}

impl<T> RefFromBytes for T
where
    T: Pod + Discriminator,
{
    fn read(data: &[u8]) -> Option<&Self> {
        let dis_len = T::DISCRIMINATOR.len();
        bytemuck::try_from_bytes(&data[dis_len..std::mem::size_of::<T>() + dis_len]).ok()
    }

    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        let dis_len = T::DISCRIMINATOR.len();
        bytemuck::try_from_bytes_mut(&mut data[dis_len..std::mem::size_of::<T>() + dis_len]).ok()
    }
}

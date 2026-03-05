use {
    crate::{Accessor, MutAccessor},
    bytemuck::{try_from_bytes, try_from_bytes_mut, AnyBitPattern, NoUninit},
    solana_program_error::ProgramError,
};

pub struct BytemuckStrategy;

impl<'a, T> Accessor<'a, T> for BytemuckStrategy
where
    T: AnyBitPattern,
{
    type Data = &'a T;

    #[inline(always)]
    fn access(data: &'a [u8]) -> Result<Self::Data, ProgramError> {
        try_from_bytes(data).map_err(|_| ProgramError::BorshIoError)
    }

    #[inline(always)]
    fn read(data: &mut &'a [u8]) -> Result<Self::Data, ProgramError> {
        let len = core::mem::size_of::<T>();
        if data.len() < len {
            return Err(ProgramError::InvalidInstructionData);
        }

        let (to_read, rem) = unsafe { data.split_at_unchecked(len) };
        *data = rem;
        try_from_bytes(to_read).map_err(|_| ProgramError::BorshIoError)
    }
}
impl<'a, T> MutAccessor<'a, T> for BytemuckStrategy
where
    T: NoUninit + AnyBitPattern,
{
    type Data = &'a mut T;

    #[inline(always)]
    fn access_mut(data: &'a mut [u8]) -> Result<Self::Data, ProgramError> {
        try_from_bytes_mut(data).map_err(|_| ProgramError::BorshIoError)
    }
}

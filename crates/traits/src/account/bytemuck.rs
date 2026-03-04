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
    fn access_and_consume(data: &'a [u8]) -> Result<(Self::Data, usize), ProgramError> {
        let len = core::mem::size_of::<T>();
        let bytes = data.get(..len).ok_or(ProgramError::BorshIoError)?;
        let value = try_from_bytes(bytes).map_err(|_| ProgramError::BorshIoError)?;
        Ok((value, len))
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

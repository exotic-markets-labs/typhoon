use borsh::BorshDeserialize;
use solana_program_error::ProgramError;

use crate::Accessor;

pub struct BorshStrategy;

impl<'a, T> Accessor<'a, T> for BorshStrategy
where
    T: BorshDeserialize + 'a,
{
    type Data = T;

    #[inline(always)]
    fn access(data: &'a [u8]) -> Result<Self::Data, ProgramError> {
        T::deserialize(&mut &data[..]).map_err(|_| ProgramError::BorshIoError)
    }
}

use {crate::Accessor, borsh::BorshDeserialize, solana_program_error::ProgramError};

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

    #[inline(always)]
    fn access_and_consume(data: &'a [u8]) -> Result<(Self::Data, usize), ProgramError> {
        let mut reader = &data[..];
        let value = T::deserialize(&mut reader).map_err(|_| ProgramError::BorshIoError)?;
        let used = data.len().saturating_sub(reader.len());
        Ok((value, used))
    }
}

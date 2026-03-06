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
    fn read(data: &mut &'a [u8]) -> Result<Self::Data, ProgramError> {
        T::deserialize(data).map_err(|_| ProgramError::BorshIoError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_simple_value() {
        let data = 42_u32.to_le_bytes();

        let value = <BorshStrategy as Accessor<'_, u32>>::access(&data).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn read_simple_value() {
        let mut data = &[7_u32.to_le_bytes(), 9_u32.to_le_bytes()].concat()[..];

        let value = <BorshStrategy as Accessor<'_, u32>>::read(&mut data).unwrap();

        assert_eq!(value, 7);
        assert_eq!(data, 9_u32.to_le_bytes());
    }

    #[test]
    fn invalid_data() {
        let data = [1_u8, 2_u8];
        let mut read_data = &data[..];

        let access_result = <BorshStrategy as Accessor<'_, u32>>::access(&data);
        let read_result = <BorshStrategy as Accessor<'_, u32>>::read(&mut read_data);

        assert_eq!(access_result, Err(ProgramError::BorshIoError));
        assert_eq!(read_result, Err(ProgramError::BorshIoError));
    }
}

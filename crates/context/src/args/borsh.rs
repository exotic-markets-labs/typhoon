use {
    crate::HandlerContext,
    borsh::BorshDeserialize,
    pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey},
    typhoon_errors::Error,
};

pub struct BorshArg<T>(pub T);

impl<T> HandlerContext<'_, '_, '_> for BorshArg<T>
where
    T: BorshDeserialize,
{
    #[inline(always)]
    fn from_entrypoint(
        _program_id: &Pubkey,
        _accounts: &mut &[AccountInfo],
        instruction_data: &mut &[u8],
    ) -> Result<Self, Error> {
        let arg = T::deserialize(instruction_data).map_err(|_| ProgramError::BorshIoError)?;

        Ok(BorshArg(arg))
    }
}

#[cfg(test)]
mod tests {
    use {super::*, borsh::BorshSerialize};

    #[test]
    fn test_borsh_arg_deserialization() {
        let mut instruction_data = [0_u8; 8];
        42_u64
            .serialize(&mut instruction_data.as_mut_slice())
            .unwrap();
        assert_eq!(instruction_data, [42, 0, 0, 0, 0, 0, 0, 0]);
        let mut accounts: &[AccountInfo] = &[];

        let mut instruction_data_slice = instruction_data.as_slice();
        let result: BorshArg<u64> = BorshArg::from_entrypoint(
            &Pubkey::default(),
            &mut accounts,
            &mut instruction_data_slice,
        )
        .unwrap_or(BorshArg(0));
        assert_eq!(result.0, 42);
        assert_eq!(instruction_data_slice.len(), 0);
    }
}

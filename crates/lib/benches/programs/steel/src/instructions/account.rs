use steel::*;

pub fn account(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let expected = u64::from_le_bytes(data[0..8].try_into().unwrap());
    if accounts.len() == expected as usize {
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

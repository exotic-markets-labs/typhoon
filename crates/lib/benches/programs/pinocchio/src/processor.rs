use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult};
use pinocchio_system::instructions::{CreateAccount, Transfer};

#[inline(always)]
pub fn process_ping() -> ProgramResult {
    Ok(())
}

#[inline(always)]
pub fn process_log() -> ProgramResult {
    msg!("Instruction: Log");
    Ok(())
}

#[inline(always)]
pub fn process_account(accounts: &[AccountInfo], expected: u64) -> ProgramResult {
    if accounts.len() == expected as usize {
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

#[inline(always)]
pub fn process_create_account(accounts: &[AccountInfo]) -> ProgramResult {
    CreateAccount {
        from: &accounts[0],
        to: &accounts[1],
        lamports: 500_000_000,
        space: 10,
        owner: &crate::ID,
    }
    .invoke()
}

#[inline(always)]
pub fn process_transfer(accounts: &[AccountInfo]) -> ProgramResult {
    Transfer {
        from: &accounts[0],
        to: &accounts[1],
        lamports: 1_000_000_000,
    }
    .invoke()
}

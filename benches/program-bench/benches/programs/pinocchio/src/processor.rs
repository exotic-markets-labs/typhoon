use {
    pinocchio::{account_info::AccountInfo, msg, ProgramResult},
    pinocchio_system::instructions::CreateAccount,
};

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
pub fn process_create_account(accounts: &[AccountInfo]) -> ProgramResult {
    let account = &accounts[1];
    CreateAccount {
        from: &accounts[0],
        to: account,
        lamports: 500_000_000,
        space: 9,
        owner: &crate::ID,
    }
    .invoke()?;
    let mut data = account.try_borrow_mut_data()?;
    data[8] = 1;

    Ok(())
}

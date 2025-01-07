use typhoon::prelude::*;
use typhoon_program::RawAccountInfo;

program_id!("Bench111111111111111111111111111111111111111");

handlers! {
    ping,
    log,
    account,
    create_account
}

#[context]
pub struct CreateAccountContext {
    pub payer: Signer,
    #[constraint(
        init,
        payer = payer,
        space = 10
    )]
    pub new_account: Mut<SystemAccount>,
    pub system: Program<System>,
}

pub fn ping() -> Result<(), ProgramError> {
    Ok(())
}

pub fn log() -> Result<(), ProgramError> {
    msg!("Instruction: Log");
    Ok(())
}

pub fn account(accounts: &[RawAccountInfo], expected: Args<[u8; 8]>) -> Result<(), ProgramError> {
    let expected = u64::from_le_bytes(*expected);
    if accounts.len() == expected as usize {
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

pub fn create_account(_ctx: CreateAccountContext) -> Result<(), ProgramError> {
    Ok(())
}

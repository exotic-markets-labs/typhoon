use {
    crate::processor::{process_create_account, process_log, process_ping, process_transfer},
    pinocchio::{
        account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError,
        pubkey::Pubkey, ProgramResult,
    },
};

nostd_panic_handler!();

entrypoint!(process_instruction);

#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [instruction, remaining @ ..] = instruction_data else {
        return Err(ProgramError::InvalidInstructionData);
    };

    match *instruction {
        // 0 - Ping
        0 => process_ping(),
        // 1 - Log
        1 => process_log(),
        // 2 - CreateAccount
        2 => process_create_account(accounts),
        3 => process_transfer(remaining, accounts),
        // Invalid instruction
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

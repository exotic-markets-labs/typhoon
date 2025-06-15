use {
    crate::{
        instruction::Instruction,
        processor::{process_create_account, process_log, process_ping},
    },
    pinocchio::{
        account_info::AccountInfo, entrypoint, nostd_panic_handler, pubkey::Pubkey, ProgramResult,
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
    let instruction = Instruction::unpack(instruction_data)?;

    match instruction {
        Instruction::Ping => process_ping(),
        Instruction::Log => process_log(),
        Instruction::CreateAccount => process_create_account(accounts),
    }
}

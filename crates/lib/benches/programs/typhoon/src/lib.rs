use typhoon::prelude::*;

program_id!("Bench111111111111111111111111111111111111111");

handlers! {
    ping,
    log
}

pub fn ping() -> Result<(), ProgramError> {
    Ok(())
}

pub fn log() -> Result<(), ProgramError> {
    msg!("Instruction: Log");
    Ok(())
}

// #[inline(always)]
// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     if program_id != &crate::ID {
//         return Err(ProgramError::IncorrectProgramId);
//     }

//     let instruction = Instruction::unpack(instruction_data)?;

//     match instruction {
//         Instruction::Ping => process_ping(),
//         Instruction::Log => process_log(),
//         Instruction::Account { expected } => process_account(accounts, expected),
//         Instruction::CreateAccount => process_create_account(accounts),
//         Instruction::Transfer => process_transfer(accounts),
//     }
// }

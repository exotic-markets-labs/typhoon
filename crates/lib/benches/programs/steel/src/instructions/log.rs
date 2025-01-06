use solana_program::msg;
use steel::*;

pub fn log() -> ProgramResult {
    msg!("Instruction: Log");
    Ok(())
}

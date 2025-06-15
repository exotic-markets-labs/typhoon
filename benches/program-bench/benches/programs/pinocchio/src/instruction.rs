use pinocchio::program_error::ProgramError;

#[derive(Clone, Debug)]
#[rustfmt::skip]
pub enum Instruction {
    Ping,
    Log,
    CreateAccount,
}

impl Instruction {
    /// Unpacks a byte buffer into a [Instruction](enum.Instruction.html).
    #[inline(always)]
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input.split_first() {
            // 0 - Ping
            Some((&0, [])) => Ok(Instruction::Ping),
            // 1 - Log
            Some((&1, [])) => Ok(Instruction::Log),
            // 2 - CreateAccount
            Some((&2, [])) => Ok(Instruction::CreateAccount),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

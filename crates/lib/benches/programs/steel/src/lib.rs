use instructions::*;
use steel::*;

mod instructions;

declare_id!("Bench111111111111111111111111111111111111111");

/// Used in generating the discriminats for instructions
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum Instructions {
    Ping = 0,
    Log = 1,
    Account = 2,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&ID, program_id, data)?;

    match ix {
        Instructions::Ping => ping()?,
        Instructions::Log => log()?,
        Instructions::Account => account(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);

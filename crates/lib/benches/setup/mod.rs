mod bench;
mod runner;

pub use bench::*;
pub use runner::*;

use litesvm::LiteSVM;
use solana_sdk::{
    account::AccountSharedData,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

pub fn setup(program_id: &Pubkey, name: &'static str) -> LiteSVM {
    // std::env::set_var("SBF_OUT_DIR", "../target/deploy");
    solana_logger::setup_with("");

    LiteSVM::new()
}

pub enum ProgramInstruction {
    Ping,
    Log,
    Account { expected: u64 },
    CreateAccount,
    Transfer,
}

/// Returns the instruction data for the given instruction.
pub fn instruction_data(instruction: ProgramInstruction) -> Vec<u8> {
    match instruction {
        ProgramInstruction::Ping => vec![0],
        ProgramInstruction::Log => vec![1],
        ProgramInstruction::Account { expected } => {
            let mut data = Vec::with_capacity(9);
            data.push(2);
            data.extend_from_slice(&expected.to_le_bytes());
            data
        }
        ProgramInstruction::CreateAccount => vec![3],
        ProgramInstruction::Transfer => vec![4],
    }
}

/// Generate a set of unique public keys.
pub fn generate_pubkeys(count: usize) -> Vec<Pubkey> {
    let mut keys = Vec::with_capacity(count);
    for _ in 0..count {
        keys.push(Pubkey::new_unique());
    }
    keys
}

/// Generates the instruction data and accounts for the
/// `ProgramInstruction::Account` instruction.
fn generate_account(program_id: Pubkey, expected: u64) -> Instruction {
    let keys = generate_pubkeys(expected as usize);
    let mut account_metas = Vec::with_capacity(keys.len());

    for key in keys {
        account_metas.push(AccountMeta::new_readonly(key, false));
    }

    Instruction {
        program_id,
        accounts: account_metas,
        data: instruction_data(ProgramInstruction::Account { expected }),
    }
}

/// Generates the instruction data and accounts for the
/// `ProgramInstruction::CreateAccount` instruction.
fn generate_create_account(program_id: Pubkey, signer: &Pubkey) -> Instruction {
    let keys = generate_pubkeys(2);
    let [key1, key2] = keys.as_slice() else {
        panic!()
    };

    let account_metas = vec![
        AccountMeta::new(*key1, true),
        AccountMeta::new(*key2, true),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    Instruction {
        program_id,
        accounts: account_metas,
        data: instruction_data(ProgramInstruction::CreateAccount),
    }
}

/// Generates the instruction data and accounts for the
/// `ProgramInstruction::Transfer` instruction.
fn generate_transfer(program_id: Pubkey, signer: &Pubkey) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(Pubkey::new_unique(), true),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    Instruction {
        program_id,
        accounts: account_metas,
        data: instruction_data(ProgramInstruction::Transfer),
    }
}

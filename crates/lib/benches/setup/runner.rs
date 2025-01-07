use solana_sdk::system_program;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
};
use std::path::PathBuf;

use super::{BenchResult, Bencher};

pub fn runner(name: &str) -> BenchResult {
    let mut so_path = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benches/programs/target/deploy"
    ));
    so_path.push(format!("{name}.so",));

    let mut bencher = Bencher::new(so_path);

    let program_id = Pubkey::from_str_const("Bench111111111111111111111111111111111111111");
    let instruction = Instruction {
        program_id,
        accounts: vec![],
        data: vec![0],
    };
    bencher.execute_ix("ping", &[instruction]);

    let instruction = Instruction {
        program_id,
        accounts: vec![],
        data: vec![1],
    };
    bencher.execute_ix("log", &[instruction]);

    let account_metas = vec![
        AccountMeta::new(bencher.payer().pubkey(), true),
        AccountMeta::new(Pubkey::new_unique(), true),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts: account_metas,
        data: vec![3],
    };
    bencher.execute_ix("create_account", &[instruction]);

    bencher.into_metrics()
}

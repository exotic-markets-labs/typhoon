use std::path::PathBuf;

use solana_sdk::{
    instruction::Instruction, message::Message, pubkey::Pubkey, transaction::Transaction,
};

use super::{instruction_data, BenchResult, Bencher};

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

    bencher.into_metrics()
}

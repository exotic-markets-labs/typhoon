use std::{collections::HashMap, path::Path};

use litesvm::LiteSVM;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::Instruction, message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    signature::Keypair, signer::Signer, transaction::Transaction,
};

pub struct Bencher {
    svm: LiteSVM,
    result: BenchResult,
    payer: Keypair,
}

impl Bencher {
    pub fn new(path: impl AsRef<Path>) -> Bencher {
        let mut svm = LiteSVM::new();
        let bytes = std::fs::read(path).unwrap();

        svm.add_program(
            Pubkey::from_str_const("Bench111111111111111111111111111111111111111"),
            &bytes,
        );

        let payer = Keypair::new();

        svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL).unwrap();

        Bencher {
            svm,
            result: BenchResult {
                binary_size: bytes.len(),
                ..Default::default()
            },
            payer,
        }
    }

    pub fn into_metrics(self) -> BenchResult {
        self.result
    }

    pub fn measure_cu(&mut self, ixs: &[Instruction]) -> u64 {
        let tx = Transaction::new(
            &[&self.payer],
            Message::new(ixs, Some(&self.payer.pubkey())),
            self.svm.latest_blockhash(),
        );

        let result = self.svm.send_transaction(tx).unwrap();
        result.compute_units_consumed
    }

    pub fn execute_ix(&mut self, ix_name: impl ToString, ixs: &[Instruction]) {
        let cu_consumed = self.measure_cu(ixs);

        self.result.metrics.insert(
            ix_name.to_string(),
            Metrics {
                cu_consumed,
                heap_alloc: 0,
                stack_size: 0,
            },
        );
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct BenchResult {
    pub metrics: HashMap<String, Metrics>,
    pub binary_size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Metrics {
    pub cu_consumed: u64,
    pub heap_alloc: u64,
    pub stack_size: u64,
}

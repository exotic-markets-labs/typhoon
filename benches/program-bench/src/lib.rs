use {
    litesvm::LiteSVM,
    serde::{Deserialize, Serialize},
    solana_hash::Hash,
    solana_keypair::Keypair,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::Transaction,
    std::{collections::HashMap, path::Path},
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

    pub fn payer(&self) -> &Keypair {
        &self.payer
    }

    pub fn hash(&self) -> Hash {
        self.svm.latest_blockhash()
    }

    pub fn into_metrics(self) -> BenchResult {
        self.result
    }

    pub fn measure_cu(&mut self, tx: Transaction) -> u64 {
        let result = self.svm.send_transaction(tx).unwrap();
        result.compute_units_consumed
    }

    pub fn execute_tx(&mut self, ix_name: impl ToString, tx: Transaction) {
        let cu_consumed = self.measure_cu(tx);

        self.result.metrics.insert(ix_name.to_string(), cu_consumed);
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct BenchResult {
    pub metrics: HashMap<String, u64>,
    pub binary_size: usize,
}

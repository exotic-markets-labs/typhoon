use litesvm::LiteSVM;
use podded::pod::PodStr;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey, signature::Keypair, signer::Signer};
use std::path::PathBuf;
use utils::{sighash, SIGHASH_GLOBAL_NAMESPACE};

mod utils;

fn read_program(name: &str) -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push(format!("../target/deploy/{name}.so"));

    std::fs::read(so_path).unwrap()
}

#[test]
fn anchor_cpi_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    // Add lever program to SVM
    let lever_id = pubkey!("E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN");
    let lever_bytes = read_program("lever");
    svm.add_program(lever_id, &lever_bytes);

    // Add hand program to SVM
    let hand_id = pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    let hand_bytes = read_program("hand");
    svm.add_program(hand_id, &hand_bytes);

    let power_kp = Keypair::new();

    // Initialize the lever
    let ix = Instruction {
        accounts: vec![
            AccountMeta::new(power_kp.pubkey(), true),
            AccountMeta::new(admin_pk, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: sighash(SIGHASH_GLOBAL_NAMESPACE, "initialize").to_vec(),
        program_id: lever_id,
    };

    let hash = svm.latest_blockhash();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp, &power_kp], hash);
    svm.send_transaction(tx).unwrap();

    // Pull the lever
    let ix = Instruction {
        accounts: vec![
            AccountMeta::new(power_kp.pubkey(), false),
            AccountMeta::new_readonly(lever_id, false),
        ],
        data: [0]
            .iter()
            .chain(bytemuck::bytes_of(&PodStr::<50>::from("Chris")).iter())
            .cloned()
            .collect(),
        program_id: hand_id,
    };

    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    let result = svm.send_transaction(tx).unwrap();

    assert!(result
        .logs
        .contains(&"Program log: Chris is pulling the power switch!".to_string()));
    assert!(result
        .logs
        .contains(&"Program log: The power is now on.".to_string()));

    // Pull it again
    let ix = Instruction {
        accounts: vec![
            AccountMeta::new(power_kp.pubkey(), false),
            AccountMeta::new_readonly(lever_id, false),
        ],
        data: [0]
            .iter()
            .chain(bytemuck::bytes_of(&PodStr::<50>::from("Ashley")).iter())
            .cloned()
            .collect(),
        program_id: hand_id,
    };

    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    let result = svm.send_transaction(tx).unwrap();

    assert!(result
        .logs
        .contains(&"Program log: Ashley is pulling the power switch!".to_string()));
    assert!(result
        .logs
        .contains(&"Program log: The power is now off!".to_string()));
}

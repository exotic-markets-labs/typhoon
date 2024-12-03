use {
    litesvm::LiteSVM,
    seeds::Counter,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::{self, Pubkey},
        signature::Keypair,
        signer::Signer,
        system_program,
        transaction::Transaction,
    },
    std::path::PathBuf,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("../../target/deploy/seeds.so");

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let program_id = pubkey::Pubkey::new_from_array(seeds::id());
    let program_bytes = read_program();

    svm.add_program(program_id, &program_bytes);

    // Create the counter
    let counter_pk = Pubkey::find_program_address(&[b"counter", b"test"], &program_id).0;

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(admin_pk, true),
            AccountMeta::new(counter_pk, false),
            AccountMeta::new(system_program::ID, false),
        ],
        data: vec![0],
    };
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(counter_pk, false)],
        data: vec![1],
    };
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&counter_pk).unwrap();
    let counter_account: &Counter = bytemuck::try_from_bytes(raw_account.data.as_slice()).unwrap();
    assert!(counter_account.count == 1);
}

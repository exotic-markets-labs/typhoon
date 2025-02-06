use {
    counter::Counter,
    litesvm::LiteSVM,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey,
        signature::Keypair,
        signer::Signer,
        system_program,
        transaction::Transaction,
    },
    std::path::PathBuf,
    typhoon::lib::RefFromBytes,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/counter.so");

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let program_id = pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    let program_bytes = read_program();

    svm.add_program(program_id, &program_bytes);

    // Create the counter
    let counter_kp = Keypair::new();
    let counter_pk = counter_kp.pubkey();
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(admin_pk, true),
            AccountMeta::new(counter_pk, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: vec![0],
    };
    let hash = svm.latest_blockhash();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp, &counter_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&counter_pk).unwrap();
    let counter_account: &Counter = Counter::read(raw_account.data.as_slice()).unwrap();
    assert!(counter_account.count == 0);

    // Increment the counter
    let ix = Instruction {
        program_id,
        accounts: vec![AccountMeta {
            pubkey: counter_pk,
            is_signer: false,
            is_writable: true,
        }],
        data: vec![1],
    };
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&counter_pk).unwrap();
    let counter_account: &Counter = Counter::read(raw_account.data.as_slice()).unwrap();
    assert!(counter_account.count == 1);

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(counter_pk, false),
            AccountMeta::new(admin_pk, false),
        ],
        data: vec![2],
    };
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&counter_pk).unwrap();
    assert_eq!(raw_account.owner, system_program::ID);
    assert_eq!(raw_account.lamports, 0);
}

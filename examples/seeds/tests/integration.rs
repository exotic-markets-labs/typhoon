use {
    litesvm::LiteSVM,
    seeds::Counter,
    solana_instruction::{AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::Transaction,
    std::path::PathBuf,
    typhoon::lib::RefFromBytes,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/seeds.so");

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();
    let random_kp = Keypair::new();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();
    svm.airdrop(&random_kp.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();

    let program_id = Pubkey::from_str_const("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    let program_bytes = read_program();

    svm.add_program(program_id, &program_bytes);

    // Create the counter
    let (counter_pk, _) = Pubkey::find_program_address(&[b"counter"], &program_id);

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(admin_pk, true),
            AccountMeta::new_readonly(Pubkey::default(), false),
            AccountMeta::new(counter_pk, false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data: vec![0],
    };
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(admin_pk, true),
            AccountMeta::new(counter_pk, false),
        ],
        data: vec![1],
    };
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&counter_pk).unwrap();
    let counter_account: &Counter = Counter::read(raw_account.data.as_slice()).unwrap();
    assert_eq!(counter_account.count, 1);

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(random_kp.pubkey(), true),
            AccountMeta::new(counter_pk, false),
        ],
        data: vec![1],
    };
    let hash = svm.latest_blockhash();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&random_kp.pubkey()), &[&random_kp], hash);
    svm.send_transaction(tx)
        .expect_err("Random signer should be able to increment");
}

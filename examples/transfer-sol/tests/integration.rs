use {
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
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/transfer_sol.so");

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    let recipient_kp = Keypair::new();
    let recipient_pk = recipient_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let program_id = pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    let program_bytes = read_program();

    svm.add_program(program_id, &program_bytes);

    let admin_balance = svm.get_balance(&admin_pk).unwrap_or_default();
    let recipient_balance = svm.get_balance(&recipient_pk).unwrap_or_default();
    assert_eq!(admin_balance, 10 * LAMPORTS_PER_SOL);
    assert_eq!(recipient_balance, 0);

    // Transfer with CPI

    let amount = LAMPORTS_PER_SOL;
    let ix = Instruction {
        accounts: vec![
            AccountMeta::new(admin_pk, true),
            AccountMeta::new(recipient_pk, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        program_id,
        data: [0]
            .iter()
            .chain(amount.to_le_bytes().iter())
            .cloned()
            .collect(),
    };

    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let admin_balance = svm.get_balance(&admin_pk).unwrap_or_default();
    let recipient_balance = svm.get_balance(&recipient_pk).unwrap_or_default();
    assert!(admin_balance > 8 * LAMPORTS_PER_SOL);
    assert_eq!(recipient_balance, LAMPORTS_PER_SOL);

    // Transfer with program

    let program_acc_kp = Keypair::new();
    let program_acc_pk = program_acc_kp.pubkey();

    let pre_ix = solana_sdk::system_instruction::create_account(
        &admin_pk,
        &program_acc_pk,
        LAMPORTS_PER_SOL,
        0,
        &program_id,
    );

    let amount = LAMPORTS_PER_SOL;
    let ix = Instruction {
        accounts: vec![
            AccountMeta::new(program_acc_pk, true),
            AccountMeta::new(admin_pk, false),
        ],
        program_id,
        data: [1]
            .iter()
            .chain(amount.to_le_bytes().iter())
            .cloned()
            .collect(),
    };

    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(
        &[pre_ix, ix],
        Some(&admin_pk),
        &[&admin_kp, &program_acc_kp],
        hash,
    );

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let admin_balance = svm.get_balance(&admin_pk).unwrap_or_default();
    let program_acc = svm.get_balance(&program_acc_pk).unwrap_or_default();
    assert!(admin_balance > 8 * LAMPORTS_PER_SOL);
    assert_eq!(program_acc, 0);
}

use {
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::{pubkey, Pubkey},
    solana_signer::Signer,
    solana_transaction::Transaction,
    std::path::PathBuf,
    transfer_sol::PodU64,
    typhoon_instruction_builder::generate_instructions_client,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/transfer_sol.so");

    std::fs::read(so_path).unwrap()
}

const ID: Pubkey = pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

generate_instructions_client!(
    transfer_sol,
    [transfer_sol_with_cpi, transfer_sol_with_program]
);

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    let recipient_kp = Keypair::new();
    let recipient_pk = recipient_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let program_bytes = read_program();

    svm.add_program(ID, &program_bytes);

    let admin_balance = svm.get_balance(&admin_pk).unwrap_or_default();
    let recipient_balance = svm.get_balance(&recipient_pk).unwrap_or_default();
    assert_eq!(admin_balance, 10 * LAMPORTS_PER_SOL);
    assert_eq!(recipient_balance, 0);

    // Transfer with CPI

    let amount = LAMPORTS_PER_SOL;
    let ix = TransferSolWithCpiInstruction {
        amount: amount.into(),
        ctx: TransferContext {
            payer: admin_pk,
            recipient: recipient_pk,
        },
        system_context: SystemContextContext {
            system: solana_system_interface::program::ID,
        },
    }
    .into_instruction();

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

    let pre_ix = solana_system_interface::instruction::create_account(
        &admin_pk,
        &program_acc_pk,
        LAMPORTS_PER_SOL,
        0,
        &ID,
    );

    let amount = LAMPORTS_PER_SOL;
    let ix = TransferSolWithProgramInstruction {
        amount: amount.into(),
        ctx: TransferContext {
            payer: program_acc_pk,
            recipient: admin_pk,
        },
    }
    .into_instruction();
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

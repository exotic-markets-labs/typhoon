use {
    litesvm::LiteSVM, solana_keypair::Keypair, solana_native_token::LAMPORTS_PER_SOL,
    solana_signer::Signer, solana_transaction::Transaction, std::path::PathBuf,
};

fn read_program(name: &str) -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push(format!("../../target/deploy/{name}.so"));

    std::fs::read(so_path).unwrap()
}

#[test]
fn lever_integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let lever_program_bytes = read_program("lever");
    let hand_program_bytes = read_program("hand");

    svm.add_program(lever_interface::ID.into(), &lever_program_bytes);
    svm.add_program(hand_interface::ID.into(), &hand_program_bytes);

    let power_kp = Keypair::new();
    let power_pk = power_kp.pubkey();

    let ix1 = lever_interface::InitializeInstruction {
        _ctx: lever_interface::InitializeLeverContext {
            power: power_pk,
            user: admin_pk,
            system_program: solana_system_interface::program::ID,
        },
    }
    .into_instruction();

    let ix2 = hand_interface::PullLeverInstruction {
        ctx: hand_interface::PullLeverContext {
            power: power_pk,
            lever_program: lever_interface::ID.into(),
        },
    }
    .into_instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix1, ix2],
        Some(&admin_pk),
        &[&admin_kp, &power_kp],
        svm.latest_blockhash(),
    );
    let logs = svm.send_transaction(tx).unwrap().logs;
    assert!(logs.contains(&"Program log: The power is now on.".to_string()));

    let ix = hand_interface::PullLeverInstruction {
        ctx: hand_interface::PullLeverContext {
            power: power_pk,
            lever_program: lever_interface::ID.into(),
        },
    }
    .into_instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&admin_pk),
        &[&admin_kp],
        svm.latest_blockhash(),
    );
    let logs = svm.send_transaction(tx).unwrap().logs;
    assert!(logs.contains(&"Program log: The power is now off!".to_string()));

    let ix = hand_interface::CheckPowerInstruction {
        ctx: hand_interface::PullLeverContext {
            power: power_pk,
            lever_program: lever_interface::ID.into(),
        },
    }
    .into_instruction();
    let _ = Transaction::new_signed_with_payer(
        &[ix],
        Some(&admin_pk),
        &[&admin_kp],
        svm.latest_blockhash(),
    );
}

use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_native_token::LAMPORTS_PER_SOL;
use solana_signer::Signer;
use solana_transaction::Transaction;
use std::path::PathBuf;

fn read_program(name: &str) -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push(format!("../../target/deploy/{name}.so"));

    println!("{:?}", so_path.to_str());

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
        power: power_pk,
        user: admin_pk,
        system_program: solana_system_interface::program::ID,
    }
    .into_instruction();

    let ix2 = hand_interface::PullLeverInstruction {
        power: power_pk,
        lever_program: lever_interface::ID.into(),
    }
    .into_instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix1, ix2],
        Some(&admin_pk),
        &[admin_kp, power_kp],
        svm.latest_blockhash(),
    );
    match svm.send_transaction(tx) {
        Ok(metadata) => println!("{}", metadata.pretty_logs()),
        Err(failed_tx) => println!("{}", failed_tx.meta.pretty_logs()),
    }

    panic!()
}

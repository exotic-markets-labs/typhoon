use {
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::{pubkey, Pubkey},
    solana_signer::Signer,
    solana_transaction::Transaction,
    std::path::PathBuf,
    typhoon_instruction_builder::generate_instructions_client,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/hello_world.so");

    std::fs::read(so_path).unwrap()
}

const ID: Pubkey = pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

generate_instructions_client!(hello_world);

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let program_bytes = read_program();

    svm.add_program(ID, &program_bytes).unwrap();

    let ix = HelloWorldInstruction {}.into_instruction();
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);

    let res = svm.send_transaction(tx).unwrap();

    assert_eq!(
        res.logs[0],
        "Program Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS invoke [1]"
    );
    assert_eq!(res.logs[1], "Program log: Hello World");
}

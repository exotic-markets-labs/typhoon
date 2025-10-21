use {
    litesvm::LiteSVM,
    solana_instruction::{AccountMeta, Instruction},
    solana_keypair::{Keypair, Signer},
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::Pubkey,
    solana_transaction::Transaction,
    std::path::PathBuf,
};

fn read_program(name: &str) -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push(format!("../target/deploy/{name}.so"));

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration() {
    let mut svm = LiteSVM::new();

    svm.add_program(misc_interface::ID, &read_program("misc"))
        .unwrap();

    let admin = Keypair::new();
    let admin_pb = admin.pubkey();

    svm.airdrop(&admin_pb, 10 * LAMPORTS_PER_SOL).unwrap();

    let ix = Instruction {
        accounts: vec![
            AccountMeta::new_readonly(Pubkey::new_unique(), false),
            AccountMeta::new_readonly(Pubkey::new_unique(), false),
            AccountMeta::new_readonly(Pubkey::new_unique(), false),
        ],
        data: vec![0],
        program_id: misc_interface::ID.into(),
    };
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&admin_pb),
        &[admin],
        svm.latest_blockhash(),
    );
    assert!(svm
        .send_transaction(tx)
        .inspect(|el| println!("{}", el.pretty_logs()))
        .is_ok());
}

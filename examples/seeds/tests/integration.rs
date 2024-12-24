use {
    litesvm::LiteSVM,
    seeds::{Counter, InitContextArgs},
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
    let random_kp = Keypair::new();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();
    svm.airdrop(&random_kp.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();

    let program_id = pubkey::Pubkey::new_from_array(seeds::id());
    let program_bytes = read_program();

    svm.add_program(program_id, &program_bytes);

    // Create the counter
    let (counter_pk, counter_bump) =
        Pubkey::find_program_address(&[b"counter", &admin_pk.to_bytes()], &program_id);

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(admin_pk, true),
            AccountMeta::new(counter_pk, false),
            AccountMeta::new(system_program::ID, false),
        ],
        data: [0]
            .iter()
            .chain(
                bytemuck::bytes_of(&InitContextArgs {
                    admin: admin_pk.to_bytes(),
                    bump: counter_bump as u64,
                })
                .iter(),
            )
            .cloned()
            .collect(),
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
    let counter_account: &Counter = bytemuck::try_from_bytes(raw_account.data.as_slice()).unwrap();
    assert!(counter_account.count == 1);

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

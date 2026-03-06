use {
    bytemuck::try_from_bytes, litesvm::LiteSVM, seeded::Counter, solana_address::Address,
    solana_keypair::Keypair, solana_native_token::LAMPORTS_PER_SOL, solana_signer::Signer,
    solana_transaction::Transaction, std::path::PathBuf, typhoon::lib::Discriminator,
    typhoon_instruction_builder::generate_instructions_client,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/seeded.so");

    std::fs::read(so_path).unwrap()
}

const ID: Address = Address::from_str_const("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

generate_instructions_client!(seeded, [initialize, increment]);

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();
    let random_kp = Keypair::new();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();
    svm.airdrop(&random_kp.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();

    let program_bytes = read_program();

    svm.add_program(ID, &program_bytes).unwrap();

    // Create the counter
    let (counter_pk, counter_bump) = Address::find_program_address(
        &Counter::derive(&admin_pk.to_bytes().into()).as_seeds(),
        &ID,
    );

    let arg = InitArgs {
        admin: admin_pk.to_bytes().into(),
        bump: counter_bump,
    };
    let ix = InitializeInstruction {
        ctx: InitContext {
            args: arg,
            payer: admin_pk,
            counter: counter_pk,
            system: solana_system_interface::program::ID,
        },
    }
    .into_instruction();
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let ix = IncrementInstruction {
        ctx: IncrementContext {
            admin: admin_pk,
            counter: counter_pk,
        },
    }
    .into_instruction();
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&counter_pk).unwrap();
    let counter_account: &Counter =
        try_from_bytes(&raw_account.data[Counter::DISCRIMINATOR.len()..]).unwrap();
    assert!(counter_account.count == 1);

    let ix = IncrementInstruction {
        ctx: IncrementContext {
            admin: random_kp.pubkey(),
            counter: counter_pk,
        },
    }
    .into_instruction();
    let hash = svm.latest_blockhash();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&random_kp.pubkey()), &[&random_kp], hash);
    svm.send_transaction(tx)
        .expect_err("Random signer should be able to increment");
}

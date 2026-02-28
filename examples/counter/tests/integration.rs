use {
    counter::Counter, litesvm::LiteSVM, solana_address::Address, solana_keypair::Keypair,
    solana_native_token::LAMPORTS_PER_SOL, solana_signer::Signer, solana_transaction::Transaction,
    std::path::PathBuf, typhoon_instruction_builder::generate_instructions_client,
};

const ID: Address = Address::from_str_const("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/counter.so");

    std::fs::read(so_path).unwrap()
}

generate_instructions_client!(counter);

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let program_bytes = read_program();

    svm.add_program(ID, &program_bytes).unwrap();

    // Create the counter
    let counter_kp = Keypair::new();
    let counter_pk = counter_kp.pubkey();
    let ix = InitializeInstruction {
        init: InitContext {
            payer: admin_pk,
            counter: counter_pk,
            system: solana_system_interface::program::ID,
        },
    }
    .into_instruction();
    let hash = svm.latest_blockhash();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp, &counter_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&counter_pk).unwrap();
    let counter_account: &Counter = bytemuck::try_from_bytes(&raw_account.data[8..]).unwrap();
    assert!(counter_account.count == 0);

    // Increment the counter
    let ix = IncrementInstruction {
        ctx: CounterMutContext {
            counter: counter_pk,
        },
    }
    .into_instruction();
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&counter_pk).unwrap();
    let counter_account: &Counter = bytemuck::try_from_bytes(&raw_account.data[8..]).unwrap();
    assert!(counter_account.count == 1);

    let ix = CloseInstruction {
        counter_mut: CounterMutContext {
            counter: counter_pk,
        },
        destination: DestinationContext {
            destination: admin_pk,
        },
    }
    .into_instruction();
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    assert!(svm.get_account(&counter_pk).is_none());
}

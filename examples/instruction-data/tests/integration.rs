use {
    instruction_data::{Buffer, InitArgs, SetValueContextArgs},
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
    so_path.push("target/deploy/instruction_data.so");

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let program_id = pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    let program_bytes = read_program();

    svm.add_program(program_id, &program_bytes);

    let buffer_a_kp = Keypair::new();
    let buffer_a_pk = buffer_a_kp.pubkey();
    let buffer_b_kp = Keypair::new();
    let buffer_b_pk = buffer_b_kp.pubkey();

    let init_args = InitArgs { value: 42 };

    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new_readonly(admin_pk, true),
                AccountMeta::new(buffer_a_pk, true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: [0]
                .iter()
                .chain(bytemuck::bytes_of(&init_args).iter())
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp, &buffer_a_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();
    let raw_account = svm.get_account(&buffer_a_pk).unwrap();
    let buffer_account = bytemuck::try_from_bytes::<Buffer>(raw_account.data.as_slice()).unwrap();
    assert!(buffer_account.value1 == init_args.value);

    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new_readonly(admin_pk, true),
                AccountMeta::new(buffer_b_pk, true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: [0]
                .iter()
                .chain(bytemuck::bytes_of(&init_args).iter())
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp, &buffer_b_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();
    let raw_account = svm.get_account(&buffer_b_pk).unwrap();
    let buffer_account = bytemuck::try_from_bytes::<Buffer>(raw_account.data.as_slice()).unwrap();
    assert!(buffer_account.value1 == init_args.value);

    let ix_a_args = SetValueContextArgs {
        value: 10,
        other_value: 5,
    };
    let more_args = 42_u64;
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new(buffer_a_pk, false)],
            data: [1]
                .iter()
                .chain(bytemuck::bytes_of(&ix_a_args).iter())
                .chain(bytemuck::bytes_of(&more_args).iter())
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();
    let raw_account = svm.get_account(&buffer_a_pk).unwrap();
    let buffer_account = bytemuck::try_from_bytes::<Buffer>(raw_account.data.as_slice()).unwrap();
    assert!(buffer_account.value1 == ix_a_args.value);
    assert!(buffer_account.value2 == more_args);

    let ix_b_args = SetValueContextArgs {
        value: 50,
        other_value: 55,
    };
    let more_args = 69_u64;
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new(buffer_b_pk, false)],
            data: [1]
                .iter()
                .chain(bytemuck::bytes_of(&ix_b_args).iter())
                .chain(bytemuck::bytes_of(&more_args).iter())
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();
    let raw_account = svm.get_account(&buffer_b_pk).unwrap();
    let buffer_account = bytemuck::try_from_bytes::<Buffer>(raw_account.data.as_slice()).unwrap();
    assert!(buffer_account.value1 == ix_b_args.value);
    assert!(buffer_account.value2 == more_args);

    let ix_a_args = SetValueContextArgs {
        value: 6,
        other_value: 11,
    };
    let ix_b_args = SetValueContextArgs {
        value: 50,
        other_value: 55,
    };
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(buffer_a_pk, false),
                AccountMeta::new(buffer_b_pk, false),
            ],
            data: [2]
                .iter()
                .chain(bytemuck::bytes_of(&ix_a_args).iter())
                .chain(bytemuck::bytes_of(&ix_b_args).iter())
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&buffer_a_pk).unwrap();
    let buffer_account = bytemuck::try_from_bytes::<Buffer>(raw_account.data.as_slice()).unwrap();
    assert!(buffer_account.value1 == ix_a_args.value);
    assert!(buffer_account.value2 == ix_a_args.value + ix_b_args.value);
}

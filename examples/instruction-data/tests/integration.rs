use typhoon::lib::RefFromBytes;
use {
    instruction_data::{Buffer, InitArgs, SetValueContextArgs},
    litesvm::LiteSVM,
    solana_instruction::{AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::pubkey,
    solana_signer::Signer,
    solana_transaction::Transaction,
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

    let init_args = InitArgs { value: 42.into() };
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new_readonly(admin_pk, true),
                AccountMeta::new(buffer_a_pk, true),
                AccountMeta::new_readonly(solana_system_interface::program::ID, false),
            ],
            data: [0]
                .iter()
                .chain(bytemuck::bytes_of(&init_args))
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp, &buffer_a_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();
    let raw_account = svm.get_account(&buffer_a_pk).unwrap();
    let buffer_account: &Buffer = Buffer::read(raw_account.data.as_slice()).unwrap();
    assert_eq!(buffer_account.value1, u64::from(init_args.value));

    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new_readonly(admin_pk, true),
                AccountMeta::new(buffer_b_pk, true),
                AccountMeta::new_readonly(solana_system_interface::program::ID, false),
            ],
            data: [0]
                .iter()
                .chain(bytemuck::bytes_of(&init_args))
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp, &buffer_b_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();
    let raw_account = svm.get_account(&buffer_b_pk).unwrap();
    let buffer_account: &Buffer = Buffer::read(raw_account.data.as_slice()).unwrap();
    assert_eq!(buffer_account.value1, u64::from(init_args.value));

    let ix_a_args = SetValueContextArgs {
        value: 10.into(),
        other_value: 5.into(),
    };
    let more_args = 42_u64;
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new(buffer_a_pk, false)],
            data: [1]
                .iter()
                .chain(bytemuck::bytes_of(&ix_a_args))
                .chain(bytemuck::bytes_of(&more_args))
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();
    let raw_account = svm.get_account(&buffer_a_pk).unwrap();
    let buffer_account: &Buffer = Buffer::read(raw_account.data.as_slice()).unwrap();
    assert_eq!(buffer_account.value1, u64::from(ix_a_args.value));
    assert_eq!(buffer_account.value2, u64::from(more_args));

    let ix_b_args = SetValueContextArgs {
        value: 50.into(),
        other_value: 55u64.into(),
    };
    let more_args = 69_u64;
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new(buffer_b_pk, false)],
            data: [1]
                .iter()
                .chain(bytemuck::bytes_of(&ix_b_args))
                .chain(bytemuck::bytes_of(&more_args))
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();
    let raw_account = svm.get_account(&buffer_b_pk).unwrap();
    let buffer_account: &Buffer = Buffer::read(raw_account.data.as_slice()).unwrap();
    assert_eq!(buffer_account.value1, u64::from(ix_b_args.value));
    assert_eq!(buffer_account.value2, u64::from(more_args));

    let ix_a_args = SetValueContextArgs {
        value: 6.into(),
        other_value: 11.into(),
    };
    let ix_b_args = SetValueContextArgs {
        value: 50.into(),
        other_value: 55.into(),
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
                .chain(bytemuck::bytes_of(&ix_a_args))
                .chain(bytemuck::bytes_of(&ix_b_args))
                .cloned()
                .collect(),
        }],
        Some(&admin_pk),
        &[&admin_kp],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&buffer_a_pk).unwrap();
    let buffer_account: &Buffer = Buffer::read(raw_account.data.as_slice()).unwrap();
    assert_eq!(buffer_account.value1, u64::from(ix_a_args.value));
    assert_eq!(
        buffer_account.value2,
        u64::from(ix_a_args.value) + u64::from(ix_b_args.value)
    );
}

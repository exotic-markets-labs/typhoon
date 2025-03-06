use {
    bytemuck::bytes_of,
    litesvm::LiteSVM,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey,
        signature::Keypair,
        signer::Signer,
        transaction::Transaction,
    },
    std::path::PathBuf,
    transfer_token::*,
    typhoon::{
        lib::{ProgramId, RefFromBytes, System},
        typhoon_program::pubkey::find_program_address,
    },
    typhoon_token::{
        find_associated_token_address, AtaTokenProgram, Mint, TokenAccount, TokenProgram,
    },
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/transfer_token.so");

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();

    let program_id = pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    let program_bytes = read_program();

    svm.add_program(program_id, &program_bytes);

    let payer_kp = Keypair::new();
    let payer_pk = payer_kp.pubkey();
    let recipient_kp = Keypair::new();
    let recipient_pk = recipient_kp.pubkey();
    let mint_kp = Keypair::new();
    let mint_pk = mint_kp.pubkey();
    let account_pk = find_associated_token_address(&mint_pk, &recipient_pk);
    let escrow_pk = find_program_address(&[&"escrow".as_ref()], &program_id).0;

    svm.airdrop(&payer_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    // Create the mint
    let minted_amount = 100000;
    svm.send_transaction(Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(payer_pk, true),
                AccountMeta::new_readonly(recipient_pk, false),
                AccountMeta::new(mint_pk, true),
                AccountMeta::new(escrow_pk.into(), false),
                AccountMeta::new(account_pk.into(), false),
                AccountMeta::new_readonly(TokenProgram::ID, false),
                AccountMeta::new_readonly(AtaTokenProgram::ID, false),
                AccountMeta::new_readonly(System::ID, false),
            ],
            data: vec![0]
                .iter()
                .chain(bytes_of(&MintFromEscrowArgs {
                    decimals: 6,
                    amount: minted_amount,
                    has_freeze_authority: 1,
                    freeze_authority: recipient_pk,
                }))
                .cloned()
                .collect(),
        }],
        Some(&payer_pk),
        &[&payer_kp, &mint_kp],
        svm.latest_blockhash(),
    ))
    .unwrap();

    let raw_account = svm.get_account(&mint_pk).unwrap();
    let mint_account = Mint::read(raw_account.data.as_slice()).unwrap();
    assert!(mint_account.mint_authority() == Some(&escrow_pk));

    let raw_account = svm.get_account(&account_pk).unwrap();
    let token_account = TokenAccount::read(raw_account.data.as_slice()).unwrap();
    assert!(
        token_account.amount() == minted_amount,
        "{} != {}",
        token_account.amount(),
        minted_amount
    );
}

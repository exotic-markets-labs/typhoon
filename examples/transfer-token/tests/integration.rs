use {
    bytemuck::bytes_of,
    litesvm::LiteSVM,
    litesvm_token::{
        get_spl_account,
        spl_token::state::{Account, Mint},
        TOKEN_ID,
    },
    solana_instruction::{AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::Transaction,
    spl_associated_token_account_client::address::get_associated_token_address,
    std::path::PathBuf,
    transfer_token::*,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/transfer_token.so");

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();

    let program_id = Pubkey::from_str_const("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    let program_bytes = read_program();

    svm.add_program(program_id, &program_bytes);

    let payer_kp = Keypair::new();
    let payer_pk = payer_kp.pubkey();
    let recipient_kp = Keypair::new();
    let recipient_pk = recipient_kp.pubkey();
    let mint_kp = Keypair::new();
    let mint_pk = mint_kp.pubkey();
    let account_pk = get_associated_token_address(&recipient_pk, &mint_pk);
    let escrow_pk = Pubkey::find_program_address(&[&"escrow".as_ref()], &program_id).0;

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
                AccountMeta::new_readonly(TOKEN_ID, false),
                AccountMeta::new_readonly(spl_associated_token_account_client::program::ID, false),
                AccountMeta::new_readonly(solana_system_interface::program::ID, false),
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

    let mint_account: Mint = get_spl_account(&svm, &mint_pk).unwrap();
    assert_eq!(mint_account.mint_authority.unwrap(), escrow_pk);

    let token_account: Account = get_spl_account(&svm, &account_pk).unwrap();

    assert_eq!(token_account.amount, minted_amount);
}

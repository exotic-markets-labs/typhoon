use {
    litesvm::LiteSVM,
    litesvm_token::{
        get_spl_account,
        spl_token::state::{Account, Mint},
        TOKEN_ID,
    },
    solana_keypair::Keypair,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::{pubkey, Pubkey},
    solana_signer::Signer,
    solana_transaction::Transaction,
    spl_associated_token_account_client::address::get_associated_token_address,
    std::path::PathBuf,
    transfer_token::MintFromEscrowArgs,
    typhoon_instruction_builder::generate_instructions_client,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/transfer_token.so");

    std::fs::read(so_path).unwrap()
}

const ID: Pubkey = pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

generate_instructions_client!(transfer_token);

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();

    let program_bytes = read_program();

    svm.add_program(ID, &program_bytes);

    let payer_kp = Keypair::new();
    let payer_pk = payer_kp.pubkey();
    let recipient_kp = Keypair::new();
    let recipient_pk = recipient_kp.pubkey();
    let mint_kp = Keypair::new();
    let mint_pk = mint_kp.pubkey();
    let account_pk = get_associated_token_address(&recipient_pk, &mint_pk);
    let escrow_pk = Pubkey::find_program_address(&[&"escrow".as_ref()], &ID).0;

    svm.airdrop(&payer_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    // Create the mint
    let minted_amount = 100000;
    svm.send_transaction(Transaction::new_signed_with_payer(
        &[MintFromEscrowInstruction {
            ctx: MintFromEscrowContextContext {
                args: MintFromEscrowArgs {
                    decimals: 6,
                    amount: minted_amount,
                },
                payer: payer_pk,
                owner: recipient_pk,
                mint: mint_pk,
                escrow: escrow_pk,
                token_account: account_pk,
                token_program: TOKEN_ID,
                ata_program: spl_associated_token_account_client::program::ID,
                system_program: solana_system_interface::program::ID,
            },
        }
        .into_instruction()],
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

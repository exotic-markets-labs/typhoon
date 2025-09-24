use {
    escrow_interface::{MakeArgs, MakeContext, MakeInstruction, TakeContext, TakeInstruction},
    litesvm::LiteSVM,
    litesvm_token::{spl_token::solana_program::pubkey::Pubkey, CreateMint, MintTo, TOKEN_ID},
    solana_keypair::Keypair,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_signer::Signer,
    solana_transaction::Transaction,
    spl_associated_token_account_client::{
        address::get_associated_token_address, instruction::create_associated_token_account,
    },
    std::path::PathBuf,
};

fn read_program(name: &str) -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push(format!("../../target/deploy/{name}.so"));

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration() {
    let mut svm = LiteSVM::new();

    svm.add_program(escrow_interface::ID, &read_program("escrow"))
        .unwrap();

    let big_admin = Keypair::new();
    let admin_pk = big_admin.pubkey();

    let maker = Keypair::new();
    let maker_pk = maker.pubkey();
    let taker = Keypair::new();
    let taker_pk = taker.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();
    svm.airdrop(&maker_pk, 10 * LAMPORTS_PER_SOL).unwrap();
    svm.airdrop(&taker_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let mint_a = CreateMint::new(&mut svm, &big_admin).send().unwrap();
    let mint_b = CreateMint::new(&mut svm, &big_admin).send().unwrap();

    let maker_ata_a = get_associated_token_address(&maker_pk, &mint_a);
    let ix = create_associated_token_account(&admin_pk, &maker_pk, &mint_a, &TOKEN_ID);

    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&big_admin], hash);
    svm.send_transaction(tx).unwrap();

    let taker_ata_b = get_associated_token_address(&taker_pk, &mint_b);
    let ix = create_associated_token_account(&admin_pk, &taker_pk, &mint_b, &TOKEN_ID);

    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&big_admin], hash);
    svm.send_transaction(tx).unwrap();

    MintTo::new(&mut svm, &big_admin, &mint_a, &maker_ata_a, 1000)
        .send()
        .unwrap();
    MintTo::new(&mut svm, &big_admin, &mint_b, &taker_ata_b, 500)
        .send()
        .unwrap();

    let escrow = Pubkey::find_program_address(
        &[b"escrow", maker_pk.as_ref(), &[0]],
        &escrow_interface::ID.into(),
    )
    .0;

    let ix = MakeInstruction {
        ctx: MakeContext {
            maker: maker_pk,
            escrow,
            mint_a,
            mint_b,
            maker_ata_a,
            vault: get_associated_token_address(&escrow, &mint_a),
            ata_program: spl_associated_token_account_client::program::ID,
            token_program: TOKEN_ID,
            system_program: Pubkey::default(),
            args: MakeArgs {
                amount: 1000,
                receive: 500,
                seed: 0,
            },
        },
    }
    .into_instruction();

    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&maker_pk), &[&maker], hash);
    svm.send_transaction(tx).unwrap();

    let ix: litesvm_token::spl_token::solana_program::instruction::Instruction = TakeInstruction {
        ctx: TakeContext {
            taker: taker_pk,
            maker: maker_pk,
            escrow,
            mint_a,
            mint_b,
            vault: get_associated_token_address(&escrow, &mint_a),
            taker_ata_a: get_associated_token_address(&taker_pk, &mint_a),
            taker_ata_b,
            maker_ata_b: get_associated_token_address(&maker_pk, &mint_b),
            ata_program: spl_associated_token_account_client::program::ID,
            token_program: TOKEN_ID,
            system_program: Pubkey::default(),
        },
    }
    .into_instruction();

    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&taker_pk), &[&taker], hash);
    svm.send_transaction(tx).unwrap();
}

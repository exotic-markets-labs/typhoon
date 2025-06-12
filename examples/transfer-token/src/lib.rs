#![no_std]

use {
    bytemuck::{AnyBitPattern, NoUninit},
    typhoon::prelude::*,
    typhoon_token::{
        spl_instructions::MintTo, AtaTokenProgram, Mint, SplCreateMint, SplCreateToken,
        TokenAccount, TokenProgram,
    },
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

nostd_panic_handler!();
no_allocator!();

handlers! {
    mint_from_escrow,
}

#[derive(AccountState, NoUninit, AnyBitPattern, Debug, Clone, Copy)]
#[repr(C)]
pub struct Escrow {}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, AnyBitPattern, NoUninit)]
pub struct MintFromEscrowArgs {
    pub amount: u64,
    pub decimals: u8,
}

#[context]
#[args(MintFromEscrowArgs)]
pub struct MintFromEscrowContext {
    pub payer: Mut<Signer>,
    pub owner: UncheckedAccount,
    #[constraint(
        init,
        payer = payer,
        mint::decimals = args.decimals,
        mint::authority = escrow.key(),
        mint::freeze_authority = owner.key()
    )]
    pub mint: Mut<InterfaceAccount<Mint>>,
    #[constraint(
        init,
        payer = payer,
        space = 8,
        seeds = [b"escrow".as_ref()],
        bump
    )]
    pub escrow: Mut<Account<Escrow>>,
    #[constraint(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub token_account: Mut<InterfaceAccount<TokenAccount>>,
    pub token_program: Interface<TokenProgram>,
    pub ata_program: Program<AtaTokenProgram>,
    pub system_program: Program<System>,
}

pub fn mint_from_escrow(ctx: MintFromEscrowContext) -> ProgramResult {
    MintTo {
        mint: ctx.mint.as_ref(),
        account: ctx.token_account.as_ref(),
        mint_authority: ctx.escrow.as_ref(),
        amount: ctx.args.amount,
    }
    .invoke_signed(&[instruction::CpiSigner::from(&seeds!(
        b"escrow".as_ref(),
        &[ctx.bumps.escrow]
    ))])?;

    Ok(())
}

use {
    bytemuck::{Pod, Zeroable},
    typhoon::prelude::*,
    typhoon_token::{spl_instructions::MintTo, AtaTokenProgram, SPLCreate, TokenProgram},
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    mint_from_escrow,
}

#[account]
pub struct Escrow {}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct MintFromEscrowArgs {
    pub freeze_authority: Pubkey,
    pub amount: u64,
    pub decimals: u8,
    pub has_freeze_authority: u8,
}

#[context]
pub struct MintFromEscrowContext {
    pub payer: Mut<Signer>,
    pub owner: UncheckedAccount,
    pub mint: Mut<UncheckedAccount>,
    #[constraint(
        init,
        payer = payer,
        space = 8,
        seeds = [b"escrow".as_ref()],
        bump
    )]
    pub escrow: Mut<Account<Escrow>>,
    pub token_account: Mut<UncheckedAccount>,
    pub token_program: Program<TokenProgram>,
    pub ata_program: Program<AtaTokenProgram>,
    pub system: Program<System>,
}

pub fn mint_from_escrow(
    ctx: MintFromEscrowContext,
    args: Args<MintFromEscrowArgs>,
) -> Result<(), ProgramError> {
    let mint = ctx.mint.create_mint(
        &Rent::get()?,
        &ctx.payer,
        &ctx.escrow.key(),
        args.decimals,
        if args.has_freeze_authority != 0 {
            Some(&args.freeze_authority)
        } else {
            None
        },
        None,
    )?;

    let token_account = ctx.token_account.create_associated_token_account(
        &ctx.payer,
        &mint,
        &ctx.owner,
        &ctx.system,
        &ctx.token_program,
    )?;

    MintTo {
        mint: mint.as_ref(),
        account: token_account.as_ref(),
        mint_authority: ctx.escrow.as_ref(),
        amount: args.amount,
    }
    .invoke_signed(&[instruction::CpiSigner::from(&seeds!(
        b"escrow".as_ref(),
        &[ctx.bumps.escrow]
    ))])?;

    Ok(())
}

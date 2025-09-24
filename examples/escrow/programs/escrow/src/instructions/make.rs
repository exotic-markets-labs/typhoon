use {
    escrow_interface::{state::Escrow, MakeArgs},
    typhoon::prelude::*,
    typhoon_token::{spl_instructions::Transfer, *},
};

#[context]
#[args(MakeArgs)]
pub struct Make {
    pub maker: Mut<Signer>,
    // TODO fix seeded and seeds
    #[constraint(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key(), &[0]],
        bump
    )]
    pub escrow: Mut<Account<Escrow>>,
    pub mint_a: InterfaceAccount<Mint>,
    pub mint_b: InterfaceAccount<Mint>,
    pub maker_ata_a: Mut<InterfaceAccount<TokenAccount>>,
    #[constraint(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: Mut<InterfaceAccount<TokenAccount>>,
    pub ata_program: Program<AtaTokenProgram>,
    pub token_program: Program<TokenProgram>,
    pub system_program: Program<System>,
}

pub fn make(ctx: Make) -> ProgramResult {
    let mut escrow_state = ctx.escrow.mut_data()?;

    *escrow_state = Escrow {
        maker: *ctx.maker.key(),
        mint_a: *ctx.mint_a.key(),
        mint_b: *ctx.mint_b.key(),
        seed: 0,
        receive: ctx.args.receive,
        bump: ctx.bumps.escrow,
    };

    Transfer {
        from: ctx.maker_ata_a.as_ref(),
        to: ctx.vault.as_ref(),
        authority: ctx.maker.as_ref(),
        amount: ctx.args.amount,
    }
    .invoke()?;

    Ok(())
}

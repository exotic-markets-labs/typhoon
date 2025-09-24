use {
    escrow_interface::state::Escrow,
    typhoon::{instruction::CpiSigner, lib::CloseAccount, prelude::*},
    typhoon_token::{
        spl_instructions::{CloseAccount as SplCloseAccount, Transfer},
        TokenAccount, TokenProgram,
    },
};

#[context]
pub struct Refund {
    pub maker: Mut<Signer>,
    pub escrow: Mut<Account<Escrow>>,
    pub mint_a: UncheckedAccount,
    #[constraint(
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: Mut<InterfaceAccount<TokenAccount>>,
    pub maker_ata_a: Mut<InterfaceAccount<TokenAccount>>,
    pub token_program: Program<TokenProgram>,
    pub system_program: Program<System>,
}

pub fn refund(ctx: Refund) -> ProgramResult {
    let escrow = ctx.escrow.data()?;
    let seed = escrow.seed.to_le_bytes();
    let seeds = seeds!(b"escrow", ctx.maker.key(), seed.as_ref());
    let signer = CpiSigner::from(&seeds);

    let amount = { ctx.vault.data()?.amount() };

    Transfer {
        from: ctx.vault.as_ref(),
        to: ctx.maker_ata_a.as_ref(),
        authority: ctx.escrow.as_ref(),
        amount,
    }
    .invoke_signed(&[signer.clone()])?;

    SplCloseAccount {
        account: ctx.vault.as_ref(),
        authority: ctx.escrow.as_ref(),
        destination: ctx.maker.as_ref(),
    }
    .invoke_signed(&[signer])?;

    ctx.escrow.close(&ctx.maker)?;

    Ok(())
}

use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    initialize,
    increment,
}

#[context]
pub struct InitContext {
    pub payer: Signer,
    pub authority: Option<Signer>,
    #[constraint(
        init,
        payer = payer,
        space = Counter::SPACE,
        seeds = [
            b"counter",
        ],
        bump
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

#[context]
pub struct IncrementContext {
    pub admin: Signer,
    #[constraint(
        has_one = admin,
        seeds = [
            b"counter",
        ],
        bump = counter.data()?.bump,
    )]
    pub counter: Mut<Account<Counter>>,
}

pub fn initialize(ctx: InitContext) -> Result<(), ProgramError> {
    assert!(ctx.authority.is_none());

    *ctx.counter.mut_data()? = Counter {
        bump: ctx.bumps.counter,
        admin: *ctx
            .authority
            .as_ref()
            .map(|a| a.key())
            .unwrap_or(ctx.payer.key()),
        count: 0,
        _padding: [0; 7],
    };

    Ok(())
}

pub fn increment(ctx: IncrementContext) -> Result<(), ProgramError> {
    ctx.counter.mut_data()?.count += 1;

    Ok(())
}

#[account]
pub struct Counter {
    pub bump: u8,
    pub admin: Pubkey,
    _padding: [u8; 7],
    pub count: u64,
}

impl Counter {
    const SPACE: usize = 8 + std::mem::size_of::<Counter>();
}

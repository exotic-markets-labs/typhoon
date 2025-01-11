use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[context]
pub struct InitContext {
    pub payer: Signer,
    #[constraint(
        init,
        payer = payer,
        space = Counter::SPACE
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

#[context]
pub struct IncrementContext {
    pub counter: Mut<Account<Counter>>,
}

handlers! {
    initialize,
    increment
}

pub fn initialize(_: InitContext) -> Result<(), ProgramError> {
    Ok(())
}

pub fn increment(ctx: IncrementContext) -> Result<(), ProgramError> {
    ctx.counter.mut_data()?.count += 1;

    Ok(())
}

#[account]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    const SPACE: usize = 8 + std::mem::size_of::<Counter>();
}

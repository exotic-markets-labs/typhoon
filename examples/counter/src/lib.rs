#![no_std]

use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[context]
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

#[context]
pub struct CounterMutContext {
    pub counter: Mut<Account<Counter>>,
}

#[context]
pub struct DestinationContext {
    pub destination: Mut<SystemAccount>,
}

handlers! {
    initialize,
    increment,
    close
}

pub fn initialize(_: InitContext) -> ProgramResult {
    Ok(())
}

pub fn increment(ctx: CounterMutContext) -> ProgramResult {
    ctx.counter.mut_data()?.count += 1;

    Ok(())
}

pub fn close(
    CounterMutContext { counter }: CounterMutContext,
    DestinationContext { destination }: DestinationContext,
) -> ProgramResult {
    counter.close(&destination)?;

    Ok(())
}

#[account]
pub struct Counter {
    pub count: u64,
}

use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[account]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    const SPACE: usize = 8 + std::mem::size_of::<Counter>();
}

#[context]
#[args(admin: Pubkey, bump: u8)]
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
        space = Counter::SPACE,
        seeds = [&args.admin],
        keys = [&args.admin],
        bump
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

pub fn main() {}

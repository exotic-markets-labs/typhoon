use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[account]
pub struct Counter {
    pub count: u64,
}

#[context]
pub struct InitContext {
    pub mint: Mut<Signer>,
    #[constraint(
        token::owner = mint
    )]
    pub counter: Mut<Account<TokenAccount>>,
}

pub fn main() {}

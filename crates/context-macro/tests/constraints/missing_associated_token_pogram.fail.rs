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
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub token_acc: Mut<Account<TokenAccount>>,
    pub token_program: Program<TokenProgram>,
    pub system_program: Program<System>,
}

pub fn main() {}

use {
    pinocchio_pubkey::{declare_id, pinocchio::pubkey::Pubkey},
    typhoon_account_macro::*,
    typhoon_accounts::*,
    typhoon_context_macro::*,
    typhoon_program_id_macro::program_id,
};

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
        seeded = [&args.admin],
        bump
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

pub fn main() {}

use {
    bytemuck::{AnyBitPattern, NoUninit},
    pinocchio_pubkey::{declare_id, pinocchio::pubkey::Pubkey},
    typhoon_account_macro::*,
    typhoon_accounts::*,
    typhoon_context_macro::*,
    typhoon_program_id_macro::program_id,
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
pub struct Counter {
    pub count: u64,
}

#[context]
#[args(admin: Pubkey)]
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        seeds = &[&args.admin],
        bump
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

pub fn main() {}

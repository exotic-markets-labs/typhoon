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
pub struct InitContext {
    pub mint: Mut<Signer>,
    #[constraint(
        token::authority = mint
    )]
    pub counter: Mut<Account<TokenAccount>>,
}

pub fn main() {}

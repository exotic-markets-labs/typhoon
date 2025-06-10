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

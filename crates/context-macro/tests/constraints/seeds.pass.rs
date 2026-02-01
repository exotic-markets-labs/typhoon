use {
    bytemuck::{AnyBitPattern, NoUninit},
    pinocchio::{
        address::{self, declare_id, Address},
        error::ProgramError,
        hint, AccountView,
    },
    typhoon_account_macro::*,
    typhoon_accounts::*,
    typhoon_context::*,
    typhoon_context_macro::*,
    typhoon_errors::*,
    typhoon_program_id_macro::program_id,
    typhoon_traits::*,
};

pub type ProgramResult<T = ()> = Result<T, Error>;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
pub struct Counter {
    pub count: u64,
}

#[context]
#[args(admin: Address, number: u64)]
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        seeds = [args.admin.as_ref(), &args.number.to_le_bytes()],
        bump
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

pub fn main() {}

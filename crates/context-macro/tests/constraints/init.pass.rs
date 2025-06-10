mod instruction {
    pub use pinocchio_pubkey::pinocchio::instruction::{Seed, Signer as CpiSigner};
}

use {
    bytemuck::{AnyBitPattern, NoUninit},
    pinocchio_pubkey::{
        declare_id,
        pinocchio::{
            account_info::AccountInfo,
            program_error::ProgramError,
            pubkey::{Pubkey, *},
            seeds,
            sysvars::{rent::Rent, Sysvar},
        },
    },
    typhoon_account_macro::*,
    typhoon_accounts::*,
    typhoon_context::HandlerContext,
    typhoon_context_macro::*,
    typhoon_errors::*,
    typhoon_program_id_macro::program_id,
    typhoon_utility_traits::SystemCpi,
};

pub type ProgramResult<T = ()> = Result<T, Error>;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
pub struct Counter {
    pub count: u64,
}

#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
pub struct CounterData {
    #[key]
    pub payer: Pubkey,
    pub bump: u8,
}

#[context]
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
        space = Counter::SPACE,
    )]
    pub counter: Mut<Account<Counter>>,
    #[constraint(
        init_if_needed,
        payer = payer,
        seeded = [payer.key()],
        bump = counter_data.data()?.bump
        has_one = payer
    )]
    pub counter_data: Mut<Account<CounterData>>,
    pub program: Program<System>,
}

pub fn main() {}

use {
    pinocchio_pubkey::{
        declare_id,
        pinocchio::{
            account_info::AccountInfo,
            program_error::ProgramError,
            pubkey::Pubkey,
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

pub type ProgramResult<T = (), E = CustomError> = Result<T, Error<E>>;

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
        space = Counter::SPACE
    )]
    pub counter: Mut<Account<Counter>>,
    pub program: Program<System>,
}

pub fn main() {}

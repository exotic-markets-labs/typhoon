#![no_std]

pub mod macros {
    pub use {
        typhoon_account_macro::*, typhoon_context_macro::*, typhoon_cpi_generator_macro::*,
        typhoon_errors_macro::*, typhoon_program_id_macro::*,
    };
}

pub mod lib {
    pub use {
        typhoon_accounts::*, typhoon_context::*, typhoon_errors::*, typhoon_traits::*,
        typhoon_utility_traits::*,
    };
}

pub mod bytes {
    pub use typhoon_utility::bytes::*;
}

pub mod instruction {
    pub use pinocchio::instruction::{InstructionAccount, InstructionView};
}

pub type ProgramResult<T = ()> = Result<T, typhoon_errors::Error>;

pub mod prelude {
    pub use {
        super::{bytes, instruction, lib::*, macros::*, ProgramResult},
        pinocchio::{
            self,
            address::{self, declare_id, MAX_SEEDS},
            cpi::{self, Seed, Signer as CpiSigner},
            default_panic_handler,
            error::{ProgramError, ToStr},
            hint,
            instruction::seeds,
            no_allocator, nostd_panic_handler, program_entrypoint,
            sysvars::{clock::Clock, fees::Fees, rent::Rent, Sysvar},
            AccountView, Address,
        },
    };
}

#![no_std]

use typhoon_errors::Error;

pub mod macros {
    pub use {
        typhoon_account_macro::*, typhoon_context_macro::*, typhoon_cpi_generator_macro::*,
        typhoon_handler_macro::*, typhoon_program_id_macro::*,
    };
}

pub mod lib {
    pub use {
        typhoon_accounts::*, typhoon_context::*, typhoon_errors::*, typhoon_utility_traits::*,
    };
}

pub mod bytes {
    pub use typhoon_utility::bytes::*;
}

pub mod instruction {
    pub use pinocchio_pubkey::pinocchio::instruction::{
        AccountMeta, Instruction, Seed, Signer as CpiSigner,
    };
}

pub type ProgramResult<T = ()> = Result<T, Error>;

pub mod prelude {
    #[cfg(not(feature = "std"))]
    pub use pinocchio::nostd_panic_handler;
    pub use {
        super::{bytes, instruction, lib::*, macros::*, ProgramResult},
        pinocchio::{
            self,
            account_info::AccountInfo,
            cpi::*,
            default_allocator, default_panic_handler, msg, no_allocator, program_entrypoint,
            program_error::{ProgramError, ToStr},
            pubkey::*,
            seeds,
            sysvars::{clock::Clock, fees::Fees, rent::Rent, Sysvar},
        },
        pinocchio_pubkey::{declare_id, from_str as pubkey_from_str},
    };
}

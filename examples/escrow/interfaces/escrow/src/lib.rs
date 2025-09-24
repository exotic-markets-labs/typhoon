#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod args;
pub mod state;

pub use args::*;
use typhoon::prelude::*;
#[cfg(feature = "client")]
use typhoon_instruction_builder::generate_instructions_client;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[cfg(feature = "client")]
generate_instructions_client!(escrow);

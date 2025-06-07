#![no_std]

#[cfg(feature = "std")]
extern crate std;

use typhoon::prelude::*;
#[cfg(feature = "client")]
use typhoon_instruction_builder::generate_instructions_client;

program_id!("Bi5N7SUQhpGknVcqPTzdFFVueQoxoUu8YTLz75J6fT8A");

#[cfg(feature = "client")]
generate_instructions_client!(hand);

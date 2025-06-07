#![no_std]

#[cfg(feature = "std")]
extern crate std;

use typhoon::prelude::*;
#[cfg(feature = "client")]
use typhoon_instruction_builder::generate_instructions_client;

program_id!("E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN");

#[account]
pub struct PowerStatus {
    pub is_on: u8,
}

impl PowerStatus {
    pub fn is_on(&self) -> bool {
        self.is_on == 1
    }
}

#[cfg(feature = "client")]
generate_instructions_client!(lever);

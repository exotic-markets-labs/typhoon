#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "cpi")]
use typhoon_instruction_builder::generate_cpi_client;
#[cfg(feature = "client")]
use typhoon_instruction_builder::generate_instructions_client;
use {
    bytemuck::{AnyBitPattern, NoUninit},
    typhoon::prelude::*,
};

program_id!("E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN");
#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
pub struct PowerStatus {
    pub is_on: u8,
}

impl PowerStatus {
    pub fn is_on(&self) -> bool {
        self.is_on == 1
    }

    pub fn change_status(&mut self) {
        self.is_on = if self.is_on == 0 { 1 } else { 0 };
    }
}

#[cfg(feature = "client")]
generate_instructions_client!(lever, [initialize]);

#[cfg(feature = "cpi")]
generate_cpi_client!(lever, [switch_power, check_power]);

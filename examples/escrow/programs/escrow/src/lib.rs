#![no_std]

use instructions::*;
use typhoon::prelude::*;

#[cfg(feature = "logging")]
use crate::errors::EscrowErrors;

mod errors;
mod instructions;

nostd_panic_handler!();
no_allocator!();

impl_error_logger!(EscrowErrors);

handlers! {
    make,
    take,
    refund
}

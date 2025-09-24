#![no_std]

use {instructions::*, typhoon::prelude::*};

pub mod instructions;

nostd_panic_handler!();
no_allocator!();

impl_error_logger!(ErrorCode);

handlers! {
    make,
    take,
    refund
}

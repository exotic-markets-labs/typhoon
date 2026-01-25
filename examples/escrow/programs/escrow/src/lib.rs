#![no_std]

use {instructions::*, typhoon::prelude::*};

pub mod instructions;

nostd_panic_handler!();
no_allocator!();
entrypoint!();

impl_error_logger!(ErrorCode);

pub const ROUTER: EntryFn = basic_router! {
    0 => make,
    1 => take,
    2 => refund,
};

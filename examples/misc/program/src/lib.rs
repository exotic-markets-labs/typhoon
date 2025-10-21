#![no_std]

pub mod intructions;

use {intructions::*, typhoon::prelude::*};

nostd_panic_handler!();
no_allocator!();

impl_error_logger!(ErrorCode);

handlers! {
    account_iter
}

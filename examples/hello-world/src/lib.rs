#![no_std]

use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    hello_world,
}

pub fn hello_world() -> ProgramResult {
    pinocchio::log::sol_log("Hello World");

    Ok(())
}

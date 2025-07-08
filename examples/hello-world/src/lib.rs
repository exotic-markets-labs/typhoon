#![no_std]

use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

nostd_panic_handler!();
no_allocator!();

handlers! {
    hello_world,
}

pub fn hello_world(ProgramIdArg(program_id): ProgramIdArg) -> ProgramResult {
    pinocchio::log::sol_log("Hello World");

    assert_eq!(program_id, &crate::ID);

    Ok(())
}

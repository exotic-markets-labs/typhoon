# Getting Started

To get started with Typhoon, you'll need a working Rust environment set up for Solana development.

## Installation

Add Typhoon to your `Cargo.toml`:

```bash
cargo add typhoon
```

## Your First Program

Here is a simple "Hello World" program using Typhoon:

```rust
#![no_std]

use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

nostd_panic_handler!();
no_allocator!();
entrypoint!();

pub const ROUTER: EntryFn = basic_router! {
    0 => hello_world
};

pub fn hello_world(ProgramIdArg(program_id): ProgramIdArg) -> ProgramResult {
    solana_msg::sol_log("Hello World");

    assert_eq!(program_id, &crate::ID);

    Ok(())
}
```

This program defines a single instruction `hello_world` that logs a message. The `basic_router!` macro handles the dispatching logic based on the instruction discriminator.

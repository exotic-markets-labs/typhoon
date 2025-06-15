#![no_std]

use {
    bytemuck::{AnyBitPattern, NoUninit},
    typhoon::prelude::*,
};

nostd_panic_handler!();
no_allocator!();

program_id!("Bench111111111111111111111111111111111111111");

handlers! {
    ping,
    log,
    create_account
}

#[inline(always)]
fn ping() -> ProgramResult {
    Ok(())
}

#[inline(always)]
fn log() -> ProgramResult {
    msg!("Instruction: Log");
    Ok(())
}

#[inline(always)]
fn create_account(ctx: CreateAccountContext) -> ProgramResult {
    ctx.account.mut_data()?.byte = 1;

    Ok(())
}

#[context]
pub struct CreateAccountContext {
    pub admin: Mut<Signer>,
    #[constraint(
        init,
        payer = admin
    )]
    pub account: Mut<Account<Data>>,
    pub system_program: Program<System>,
}

#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
pub struct Data {
    pub byte: u8,
}

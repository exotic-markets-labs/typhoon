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
    create_account,
    transfer,
    unchecked_accounts
}

fn ping() -> ProgramResult {
    Ok(())
}

fn log() -> ProgramResult {
    msg!("Instruction: Log");
    Ok(())
}

fn create_account(ctx: CreateAccountContext) -> ProgramResult {
    ctx.account.mut_data()?.byte = 1;

    Ok(())
}

fn transfer(Arg(amount): Arg<[u8; 8]>, ctx: TransferContext) -> ProgramResult {
    ctx.admin
        .transfer(&ctx.account, u64::from_le_bytes(*amount))
}

fn unchecked_accounts(ctx: UncheckedAccountsContext) -> ProgramResult {
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

#[context]
pub struct TransferContext {
    pub admin: Mut<Signer>,
    pub account: Mut<SystemAccount>,
    pub system_program: Program<System>,
}

#[context]
pub struct UncheckedAccountsContext {
    pub account1: UncheckedAccount,
    pub account2: UncheckedAccount,
    pub account3: UncheckedAccount,
    pub account4: UncheckedAccount,
    pub account5: UncheckedAccount,
    pub account6: UncheckedAccount,
    pub account7: UncheckedAccount,
    pub account8: UncheckedAccount,
    pub account9: UncheckedAccount,
    pub account10: UncheckedAccount,
}

#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
pub struct Data {
    pub byte: u8,
}

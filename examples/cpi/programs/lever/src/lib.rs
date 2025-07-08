#![no_std]

use {lever_interface::PowerStatus, typhoon::prelude::*};

nostd_panic_handler!();
no_allocator!();

impl_error_logger!(ErrorCode);

handlers! {
    initialize,
    switch_power,
    check_power
}

pub fn initialize(_ctx: InitializeLever) -> ProgramResult {
    Ok(())
}

pub fn switch_power(ctx: SetPowerStatus) -> ProgramResult {
    let mut power = ctx.power.mut_data()?;
    power.change_status();

    match power.is_on() {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };
    Ok(())
}

pub fn check_power(ctx: CheckStatus) -> ProgramResult {
    let power = ctx.power.as_ref().unwrap().data()?;

    match power.is_on() {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };

    Ok(())
}

#[context]
pub struct InitializeLever {
    #[constraint(
        init,
        payer = user
    )]
    pub power: Mut<Account<PowerStatus>>,
    pub user: Mut<Signer>,
    pub system_program: Program<System>,
}

#[context]
pub struct SetPowerStatus {
    pub power: Mut<Account<PowerStatus>>,
}

#[context]
#[args(random: u64)]
pub struct CheckStatus {
    pub power: Option<Account<PowerStatus>>,
}

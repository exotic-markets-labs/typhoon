#![no_std]

use {
    lever_interface::{
        CheckPowerCpi, CheckStatusArgs, LeverInterfaceProgram, PowerStatus, SwitchPowerCpi,
    },
    typhoon::prelude::*,
};

nostd_panic_handler!();
no_allocator!();

impl_error_logger!(ErrorCode);

handlers! {
    pull_lever,
    check_power
}

pub fn check_power(ctx: PullLever) -> ProgramResult {
    CheckPowerCpi {
        arg_0: &CheckStatusArgs { random: 1 },
        power: None,
        program: ctx.lever_program.as_ref(),
    }
    .invoke()
}

pub fn pull_lever(ctx: PullLever) -> ProgramResult {
    SwitchPowerCpi {
        power: ctx.power.as_ref(),
        program: ctx.lever_program.as_ref(),
    }
    .invoke()?;
    Ok(())
}

#[context]
pub struct PullLever {
    pub power: Mut<Account<PowerStatus>>,
    pub lever_program: Program<LeverInterfaceProgram>,
}

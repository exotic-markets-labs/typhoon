#![no_std]

use {
    lever_interface::{
        CheckPowerCpi, CheckStatusArgs, CheckStatusContext, LeverInterfaceProgram, PowerStatus,
        SetPowerStatusContext, SwitchPowerCpi,
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
    let return_data = CheckPowerCpi {
        ctx: CheckStatusContext {
            args: &CheckStatusArgs { random: 1 },
            power: None,
        },
        program: ctx.lever_program.as_ref(),
    }
    .invoke()?;
    assert_eq!(return_data, 1);
    Ok(())
}

pub fn pull_lever(ctx: PullLever) -> ProgramResult {
    SwitchPowerCpi {
        ctx: SetPowerStatusContext {
            power: ctx.power.as_ref(),
        },
        program: ctx.lever_program.key(),
    }
    .invoke()?;
    Ok(())
}

#[context]
pub struct PullLever {
    pub power: Mut<Account<PowerStatus>>,
    pub lever_program: Program<LeverInterfaceProgram>,
}

#![no_std]

use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

nostd_panic_handler!();
no_allocator!();

handlers! {
    pull_lever,
}

pub fn pull_lever(ctx: PullLever, name: Arg<[u8; 16]>) -> ProgramResult {
    let last_char = name.iter().position(|&x| x == 0).unwrap_or(name.len());
    crate::lever_cpi::SwitchPower {
        power: ctx.power.as_ref(),
        name: core::str::from_utf8(&name[..last_char])
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    }
    .invoke()
}

#[context]
pub struct PullLever {
    pub power: Mut<BorshAccount<crate::lever_cpi::PowerStatus>>,
    pub lever_program: Program<crate::lever_cpi::LeverProgram>,
}

anchor_cpi!("../../idls/lever.json");

use {podded::pod::PodStr, typhoon::prelude::*};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    pull_lever,
}

pub fn pull_lever(ctx: PullLever, name: Args<PodStr<50>>) -> Result<(), ProgramError> {
    crate::cpi::SwitchPower {
        power: ctx.power.as_ref(),
        name: name.as_ref(),
    }
    .invoke()
}

#[context]
pub struct PullLever {
    pub power: Mut<BorshAccount<crate::cpi::PowerStatus>>,
    pub lever_program: Program<crate::cpi::LeverProgram>,
}

anchor_cpi!("../../idls/lever.json");

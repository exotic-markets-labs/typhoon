use {podded::pod::PodStr, typhoon::prelude::*};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    pull_lever,
}

pub fn pull_lever(ctx: PullLever, name: Args<PodStr<50>>) -> Result<(), ProgramError> {
    crate::lever_cpi::SwitchPower {
        power: ctx.power.as_ref(),
        name: name.as_ref(),
    }
    .invoke()
}

#[context]
pub struct PullLever {
    pub power: Mut<BorshAccount<crate::lever_cpi::PowerStatus>>,
    pub lever_program: Program<crate::lever_cpi::LeverProgram>,
}

anchor_cpi!("../../idls/lever.json");

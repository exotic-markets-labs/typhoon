use typhoon::prelude::*;

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    pull_lever,
}

pub fn pull_lever() -> Result<(), ProgramError> {
    msg!("Hello World");

    Ok(())
}

#[context]
pub struct PullLever {
    pub power: Mut<Account<PowerStatus>>,
    pub lever_program: Program<Lever>,
}

anchor_cpi!("/home/aursen/crayfish/examples/anchor-cpi/idls/lever.json");

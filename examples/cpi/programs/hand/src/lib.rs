#![no_std]

use lever_interface::{LeverInterfaceProgram, PowerStatus};
use typhoon::prelude::*;

nostd_panic_handler!();
no_allocator!();

impl_error_logger!(ErrorCode);

const ID: Pubkey = hand_interface::ID;

handlers! {
    pull_lever
}

pub fn pull_lever(ctx: PullLever) -> ProgramResult {
    //TODO cpi
    invoke(
        &instruction::Instruction {
            accounts: &[instruction::AccountMeta::new(&crate::ID, false, false)],
            data: &[3],
            program_id: ctx.lever_program.key(),
        },
        &[],
    )?;
    Ok(())
}

#[context]
pub struct PullLever {
    pub power: Mut<Account<PowerStatus>>,
    pub lever_program: Program<LeverInterfaceProgram>,
}

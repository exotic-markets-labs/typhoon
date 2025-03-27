use {crate::ProgramId, pinocchio::pubkey::Pubkey};

pub struct System;

impl ProgramId for System {
    const ID: Pubkey = pinocchio_system::ID;
}

use {pinocchio::pubkey::Pubkey, typhoon_definitions::ProgramId};

pub struct System;

impl ProgramId for System {
    const ID: Pubkey = pinocchio_system::ID;
}

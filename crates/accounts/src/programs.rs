use {crate::ProgramId, typhoon_program::system_program};

pub struct System;

impl ProgramId for System {
    const ID: typhoon_program::pubkey::Pubkey = system_program::ID;
}

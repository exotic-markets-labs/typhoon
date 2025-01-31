use {typhoon_program::system_program, typhoon_traits::ProgramId};

pub struct System;

impl ProgramId for System {
    const ID: typhoon_program::pubkey::Pubkey = system_program::ID;
}

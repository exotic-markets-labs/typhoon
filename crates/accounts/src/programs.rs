use {solana_address::Address, typhoon_traits::ProgramId};

pub struct System;

impl ProgramId for System {
    const ID: Address = Address::new_from_array([0; 32]);
}

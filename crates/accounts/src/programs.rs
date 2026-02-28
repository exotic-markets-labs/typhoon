use {
    solana_address::{address_eq, Address},
    typhoon_traits::CheckProgramId,
};

pub struct System;

impl CheckProgramId for System {
    #[inline(always)]
    fn address_eq(program_id: &Address) -> bool {
        address_eq(program_id, &Address::new_from_array([0; 32]))
    }
}

use {
    misc_interface::{pda::RANDOM_PDA, state::RandomData},
    typhoon::prelude::*,
};

#[context]
pub struct Simple {
    #[constraint(
        assert = account.data()?.counter == 1,
        address = &RANDOM_PDA.0
    )]
    pub account: Account<RandomData>,
}

pub fn assert(_: Simple) -> ProgramResult {
    Ok(())
}

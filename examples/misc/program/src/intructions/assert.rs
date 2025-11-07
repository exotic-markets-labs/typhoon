use {misc_interface::{pda::RANDOM_PDA, state::RandomData},, typhoon::prelude::*};

#[context]
pub struct Simple {
    #[constraint(
        assert = account.data()?.counter == 1,
        address = &RANDOM_PDA
    )]
    pub account: Account<RandomData>,
}

pub fn assert(_: Simple) -> ProgramResult {
    Ok(())
}

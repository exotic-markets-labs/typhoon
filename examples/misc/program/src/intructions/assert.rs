use {misc_interface::state::RandomData, typhoon::prelude::*};

#[context]
pub struct Simple {
    #[constraint(
        assert = account.data()?.counter == 1
    )]
    pub account: Account<RandomData>,
}

pub fn assert(_: Simple) -> ProgramResult {
    Ok(())
}

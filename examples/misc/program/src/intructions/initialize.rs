use {misc_interface::state::RandomData, typhoon::prelude::*};

#[context]
pub struct Initialize {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
        address = &RANDOM_PDA
    )]
    pub account: Mut<SignerNoCheck<Account<RandomData>>>,
    pub system_program: Program<System>,
}

pub fn initialize(context: Initialize) -> ProgramResult {
    context.account.mut_data()?.counter = 1;
    Ok(())
}

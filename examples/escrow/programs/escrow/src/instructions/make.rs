use escrow_interface::state::Escrow;
use typhoon::prelude::*;
use typhoon_token::*;

#[context]
pub struct Make {
    pub maker: Signer,
    pub escrow: Mut<Account<Escrow>>,
    pub mint_a: InterfaceAccount<Mint>,
    pub mint_b: InterfaceAccount<Mint>,
    // #[constraint(
    //     init_if_needed,
    //     payer = payer,
    //     associated_token::mint = mint,
    //     associated_token::authority = owner
    // )]
    pub maker_ata_a: InterfaceAccount<TokenAccount>,
    // pub vault: &'a AccountInfo,
    pub ata_program: Program<AtaTokenProgram>,
    pub token_program: Program<TokenProgram>,
    pub system_program: Program<System>,
}

pub fn make() -> ProgramResult {
    Ok(())
}

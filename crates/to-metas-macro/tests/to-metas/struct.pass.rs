use typhoon_accounts::{Meta, Signer, SystemAccount};
use typhoon_to_metas_macro::ToMetas;

pub struct TestBumps {
    pub bump: u8,
}

#[derive(ToMetas)]
pub struct TestContext<'info> {
    pub account1: Signer<'info>,
    pub account2: SystemAccount<'info>,
    pub bumps: TestBumps,
}

pub fn main() {}

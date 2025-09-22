use typhoon::prelude::*;

#[derive(TyphoonError)]
pub enum EscrowErrors {
    #[msg("Lamport balance below rent-exempt threshold")]
    NotRentExempt = 200,
    #[msg("Account is not a signer")]
    NotSigner,
    #[msg("Account owner is invalid")]
    InvalidOwner,
    #[msg("Account is not owned by the program")]
    NotProgramOwner,
    #[msg("Account Address is invalid")]
    InvalidAddress,
}

use {
    num_derive::{FromPrimitive, ToPrimitive},
    thiserror::Error,
};

#[derive(Debug, Error, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum ErrorCode {
    #[error("Program is not executable")]
    InvalidProgramExecutable = 3000,

    #[error("Account is initialized yet")]
    AccountNotInitialized,

    #[error("The given account is not mutable")]
    AccountNotMutable,

    #[error("Account is not a signer")]
    AccountNotSigner,

    #[error("The current owner of this account is not the expected one")]
    AccountOwnedByWrongProgram,

    #[error("Discriminator did not match what was expected")]
    AccountDiscriminatorMismatch,

    #[error("has_one constraint violated")]
    HasOneConstraint,

    #[error("Cannot initialize a program account with the payer account")]
    TryingToInitPayerAsProgramAccount,

    #[error("Token constraint was violated")]
    TokenConstraintViolated,
}

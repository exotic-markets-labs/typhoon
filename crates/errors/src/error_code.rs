use pinocchio::program_error::{ProgramError, ToStr};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidProgramExecutable = 100,
    AccountNotInitialized,
    AccountNotMutable,
    AccountNotSigner,
    AccountOwnedByWrongProgram,
    AccountDiscriminatorMismatch,
    HasOneConstraint,
    TryingToInitPayerAsProgramAccount,
    TokenConstraintViolated,
    BufferFull,
}

impl TryFrom<u32> for ErrorCode {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(ErrorCode::InvalidProgramExecutable),
            101 => Ok(ErrorCode::AccountNotInitialized),
            102 => Ok(ErrorCode::AccountNotMutable),
            103 => Ok(ErrorCode::AccountNotSigner),
            104 => Ok(ErrorCode::AccountOwnedByWrongProgram),
            105 => Ok(ErrorCode::AccountDiscriminatorMismatch),
            106 => Ok(ErrorCode::HasOneConstraint),
            107 => Ok(ErrorCode::TryingToInitPayerAsProgramAccount),
            108 => Ok(ErrorCode::TokenConstraintViolated),
            109 => Ok(ErrorCode::BufferFull),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl From<ErrorCode> for ProgramError {
    #[inline(always)]
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl ToStr for ErrorCode {
    fn to_str<E>(&self) -> &'static str
    where
        E: 'static + ToStr + TryFrom<u32>,
    {
        match self {
            ErrorCode::InvalidProgramExecutable => "Error: Program is not executable",
            ErrorCode::AccountNotInitialized => "Error: Account is not initialized yet",
            ErrorCode::AccountNotMutable => "Error: The given account is not mutable",
            ErrorCode::AccountNotSigner => "Error: Account is not a signer",
            ErrorCode::AccountOwnedByWrongProgram => {
                "Error: The current owner of this account is not the expected one"
            }
            ErrorCode::AccountDiscriminatorMismatch => {
                "Error: Discriminator did not match what was expected"
            }
            ErrorCode::HasOneConstraint => "Error: has_one constraint violated",
            ErrorCode::TryingToInitPayerAsProgramAccount => {
                "Error: Cannot initialize a program account with the payer account"
            }
            ErrorCode::TokenConstraintViolated => "Error: Token constraint was violated",
            ErrorCode::BufferFull => "Error: Buffer is full",
        }
    }
}

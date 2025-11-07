use pinocchio::program_error::{ProgramError, ToStr};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidProgramExecutable = 100,
    AccountNotSigner,
    AccountDiscriminatorMismatch,
    HasOneConstraint,
    AssertConstraint,
    TryingToInitPayerAsProgramAccount,
    TokenConstraintViolated,
    BufferFull,
    InvalidReturnData,
    UnknownInstruction,
    InvalidDataLength,
    InvalidDataAlignment,
}

impl TryFrom<u32> for ErrorCode {
    type Error = ProgramError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(ErrorCode::InvalidProgramExecutable),
            101 => Ok(ErrorCode::AccountNotSigner),
            102 => Ok(ErrorCode::AccountDiscriminatorMismatch),
            103 => Ok(ErrorCode::HasOneConstraint),
            104 => Ok(ErrorCode::AssertConstraint),
            105 => Ok(ErrorCode::TryingToInitPayerAsProgramAccount),
            106 => Ok(ErrorCode::TokenConstraintViolated),
            107 => Ok(ErrorCode::BufferFull),
            108 => Ok(ErrorCode::InvalidReturnData),
            109 => Ok(ErrorCode::UnknownInstruction),
            110 => Ok(ErrorCode::InvalidDataLength),
            111 => Ok(ErrorCode::InvalidDataAlignment),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl From<ErrorCode> for ProgramError {
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
            ErrorCode::AccountNotSigner => "Error: Account is not a signer",
            ErrorCode::AccountDiscriminatorMismatch => {
                "Error: Discriminator did not match what was expected"
            }
            ErrorCode::HasOneConstraint => "Error: has_one constraint violated",
            ErrorCode::AssertConstraint => "Error: assert constraint violated",
            ErrorCode::TryingToInitPayerAsProgramAccount => {
                "Error: Cannot initialize a program account with the payer account"
            }
            ErrorCode::TokenConstraintViolated => "Error: Token constraint was violated",
            ErrorCode::BufferFull => "Error: Buffer is full",
            ErrorCode::InvalidReturnData => "Error: The return data is invalid",
            ErrorCode::UnknownInstruction => "Error: Unknown instruction",
            ErrorCode::InvalidDataLength => "Error: Invalid data length",
            ErrorCode::InvalidDataAlignment => "Error: Invalid data alignment",
        }
    }
}

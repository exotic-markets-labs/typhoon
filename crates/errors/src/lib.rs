use {
    num_traits::{FromPrimitive, ToPrimitive},
    thiserror::Error,
    typhoon_program::{msg, program_error::ProgramError},
};

/// Maybe rework with thiserror 2.0
#[derive(Debug, Error)]
pub enum Error {
    #[error("Program is not executable")]
    InvalidProgramExecutable,

    #[error("Account is initialized yet")]
    AccountNotInitialized,

    #[error("The given account is not mutable")]
    AccountNotMutable,

    #[error("Account is not a signer")]
    AccountNotSigner,

    #[error("The current owner of this account is not the expected one")]
    AccountOwnedByWrongProgram,
}

impl FromPrimitive for Error {
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            3000 => Some(Error::InvalidProgramExecutable),
            3001 => Some(Error::AccountNotInitialized),
            3002 => Some(Error::AccountNotMutable),
            3003 => Some(Error::AccountNotSigner),
            3004 => Some(Error::AccountOwnedByWrongProgram),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}

impl ToPrimitive for Error {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Error::InvalidProgramExecutable => Some(3000),
            Error::AccountNotInitialized => Some(3001),
            Error::AccountNotMutable => Some(3002),
            Error::AccountNotSigner => Some(3003),
            Error::AccountOwnedByWrongProgram => Some(3004),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_i64().map(|n| n as u64)
    }
}

impl From<Error> for ProgramError {
    fn from(value: Error) -> Self {
        let error_msg = format!("[ERROR] {}", value.to_string());
        msg!(&error_msg);
        ProgramError::Custom(value.to_u32().unwrap())
    }
}

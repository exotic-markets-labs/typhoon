#![no_std]

mod error_code;
mod extension;

pub use {error_code::*, extension::*};
use {
    solana_address::error::AddressError,
    solana_program_error::{ProgramError, ToStr},
};

pub struct Error {
    error: ProgramError,
    account_name: Option<&'static str>,
}

impl Error {
    pub fn new(error: impl Into<ProgramError>) -> Self {
        Error {
            error: error.into(),
            account_name: None,
        }
    }

    pub fn with_account(mut self, name: &'static str) -> Self {
        self.account_name = Some(name);
        self
    }

    pub fn account_name(&self) -> Option<&str> {
        self.account_name
    }

    pub fn to_str<E>(&self) -> &'static str
    where
        E: ToStr + TryFrom<u32> + 'static,
    {
        if let ProgramError::Custom(code) = self.error {
            if (100..200).contains(&code) {
                return self.error.to_str::<ErrorCode>();
            }
        }
        self.error.to_str::<E>()
    }
}

impl From<ProgramError> for Error {
    fn from(error: ProgramError) -> Self {
        Error {
            error,
            account_name: None,
        }
    }
}

impl From<ErrorCode> for Error {
    fn from(value: ErrorCode) -> Self {
        Error {
            error: value.into(),
            account_name: None,
        }
    }
}

impl From<Error> for ProgramError {
    fn from(value: Error) -> Self {
        value.error
    }
}

impl From<AddressError> for Error {
    fn from(value: AddressError) -> Self {
        Self {
            error: value.into(),
            account_name: None,
        }
    }
}

#[cfg(feature = "logging")]
pub type LogError = ErrorCode;

#[cfg(feature = "logging")]
#[cold]
pub fn log_error<E>(error: &Error)
where
    E: ToStr + TryFrom<u32> + 'static,
{
    solana_program_log::log(error.to_str::<E>());

    if let Some(account_name) = error.account_name() {
        let mut logger = solana_program_log::Logger::<50>::default();
        logger.append("Account origin: ");
        logger.append(unsafe { str::from_utf8_unchecked(account_name.as_bytes()) });
        logger.log();
    }
}

#[macro_export]
macro_rules! require {
    ( $constraint:expr, $error:expr ) => {
        if pinocchio::hint::unlikely(!$constraint) {
            return Err($error.into());
        }
    };
}

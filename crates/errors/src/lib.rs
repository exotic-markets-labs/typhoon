#![no_std]

mod error_code;
mod extension;

use pinocchio::program_error::{ProgramError, ToStr};
pub use {error_code::*, extension::*};

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
}

impl ToStr for Error {
    fn to_str<E>(&self) -> &'static str
    where
        E: 'static + ToStr + TryFrom<u32>,
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

#[macro_export]
macro_rules! impl_error_logger {
    ($error:ident) => {
        #[cfg(feature = "logging")]
        #[cold]
        fn log_error(error: &Error) {
            pinocchio::log::sol_log(error.to_str::<$error>());
            if let Some(account_name) = error.account_name() {
                let mut buffer = [bytes::UNINIT_BYTE; 50];
                let total_len = core::cmp::min(account_name.len() + 16, 50);
                bytes::write_bytes(&mut buffer[..16], b"Account origin: ");
                bytes::write_bytes(&mut buffer[16..total_len], account_name.as_bytes());
                pinocchio::log::sol_log(unsafe {
                    core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                        buffer.as_ptr() as _,
                        total_len,
                    ))
                });
            }
        }
    };
}

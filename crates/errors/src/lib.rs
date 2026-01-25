#![no_std]

mod error_code;
mod extension;

use {
    core::marker::PhantomData,
    solana_address::error::AddressError,
    solana_program_error::{ProgramError, ToStr},
};
pub use {error_code::*, extension::*};

pub struct Error<E = ErrorCode> {
    error: ProgramError,
    account_name: Option<&'static str>,
    _phantom: PhantomData<E>,
}

impl<E> Error<E> {
    pub fn new(error: impl Into<ProgramError>) -> Self {
        Error {
            error: error.into(),
            account_name: None,
            _phantom: PhantomData,
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

impl<E> ToStr for Error<E>
where
    E: ToStr + TryFrom<u32> + 'static,
{
    fn to_str(&self) -> &'static str {
        if let ProgramError::Custom(code) = self.error {
            if (100..200).contains(&code) {
                return self.error.to_str::<ErrorCode>();
            }
        }
        self.error.to_str::<E>()
    }
}

impl<E> From<ProgramError> for Error<E> {
    fn from(error: ProgramError) -> Self {
        Error {
            error,
            account_name: None,
            _phantom: PhantomData,
        }
    }
}

impl<E> From<ErrorCode> for Error<E> {
    fn from(value: ErrorCode) -> Self {
        Error {
            error: value.into(),
            account_name: None,
            _phantom: PhantomData,
        }
    }
}

impl<E> From<Error<E>> for ProgramError {
    fn from(value: Error<E>) -> Self {
        value.error
    }
}

impl<E> From<AddressError> for Error<E> {
    fn from(value: AddressError) -> Self {
        Self {
            error: value.into(),
            account_name: None,
            _phantom: PhantomData,
        }
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

#[macro_export]
macro_rules! require {
    ( $constraint:expr, $error:expr ) => {
        if pinocchio::hint::unlikely(!$constraint) {
            return Err($error.into());
        }
    };
}

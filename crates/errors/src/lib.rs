mod default_custom;
mod error_code;

use {
    num_traits::{FromPrimitive, ToPrimitive},
    pinocchio::program_error::ProgramError,
    std::fmt::Display,
};

pub use default_custom::*;
pub use error_code::*;

pub enum ErrorType<T = CustomError>
where
    T: Display + FromPrimitive + ToPrimitive,
{
    Solana(ProgramError),
    Typhoon(ErrorCode),
    Custom(T),
}

pub struct Error<T = CustomError>
where
    T: Display + FromPrimitive + ToPrimitive,
{
    error: ErrorType<T>,
    account_name: Option<String>,
}

impl<T> Error<T>
where
    T: Display + FromPrimitive + ToPrimitive,
{
    pub fn new_solana(error: ProgramError) -> Self {
        Self {
            error: ErrorType::Solana(error),
            account_name: None,
        }
    }

    pub fn new_typhoon(error: ErrorCode) -> Self {
        Self {
            error: ErrorType::Typhoon(error),
            account_name: None,
        }
    }

    pub fn new_custom(error: T) -> Self {
        Self {
            error: ErrorType::Custom(error),
            account_name: None,
        }
    }

    pub fn with_account(mut self, name: String) -> Self {
        self.account_name = Some(name);
        self
    }
}

impl<T> From<Error<T>> for ProgramError
where
    T: Display + FromPrimitive + ToPrimitive,
{
    fn from(value: Error<T>) -> Self {
        match value.error {
            ErrorType::Solana(program_error) => program_error,
            ErrorType::Typhoon(error_code) => ProgramError::Custom(error_code.to_u32().unwrap()),
            ErrorType::Custom(custom) => ProgramError::Custom(custom.to_u32().unwrap()),
        }
    }
}

impl<T> From<ErrorCode> for Error<T>
where
    T: Display + FromPrimitive + ToPrimitive,
{
    fn from(value: ErrorCode) -> Self {
        Error::new_typhoon(value)
    }
}

impl<T> From<ProgramError> for Error<T>
where
    T: Display + FromPrimitive + ToPrimitive,
{
    fn from(value: ProgramError) -> Self {
        Error::new_solana(value)
    }
}

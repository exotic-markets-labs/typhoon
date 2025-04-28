mod default_custom;
mod error_code;
mod extension;

pub use {default_custom::*, error_code::*, extension::*};
use {
    num_traits::{FromPrimitive, ToPrimitive},
    pinocchio::program_error::ProgramError,
    std::fmt::Display,
};

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

    pub fn with_account(mut self, name: impl ToString) -> Self {
        self.account_name = Some(name.to_string());
        self
    }
}

impl<T> From<Error<T>> for ProgramError
where
    T: Display + FromPrimitive + ToPrimitive,
{
    fn from(value: Error<T>) -> Self {
        //todo log here
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

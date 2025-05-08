mod default_custom;
mod error_code;
mod extension;

use pinocchio::{
    log::sol_log,
    program_error::{ProgramError, ToStr},
};
pub use {default_custom::*, error_code::*, extension::*};

pub enum ErrorType<T>
where
    T: 'static + ToStr + TryFrom<u32>,
{
    Solana(ProgramError),
    Typhoon(ErrorCode),
    Custom(T),
}

pub struct Error<T = CustomError>
where
    T: 'static + ToStr + TryFrom<u32>,
{
    error: ErrorType<T>,
    account_name: Option<String>,
}

impl<T> Error<T>
where
    T: 'static + ToStr + TryFrom<u32>,
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

impl<T> From<ErrorCode> for Error<T>
where
    T: 'static + ToStr + TryFrom<u32>,
{
    fn from(value: ErrorCode) -> Self {
        Error::new_typhoon(value)
    }
}

impl<T> From<ProgramError> for Error<T>
where
    T: 'static + ToStr + TryFrom<u32>,
{
    fn from(value: ProgramError) -> Self {
        Error::new_solana(value)
    }
}

impl<T> From<Error<T>> for ProgramError
where
    T: 'static + ToStr + TryFrom<u32> + Into<u32>,
{
    fn from(value: Error<T>) -> Self {
        let program_error = match value.error {
            ErrorType::Solana(program_error) => {
                sol_log(program_error.to_str::<CustomError>());
                program_error
            }
            ErrorType::Typhoon(error_code) => {
                sol_log(error_code.to_str::<ErrorCode>());
                ProgramError::Custom(error_code.into())
            }
            ErrorType::Custom(custom_error) => {
                sol_log(custom_error.to_str::<T>());
                ProgramError::Custom(custom_error.into())
            }
        };
        if let Some(account_name) = value.account_name {
            sol_log(&format!("Account origin: {account_name}"));
        }

        program_error
    }
}

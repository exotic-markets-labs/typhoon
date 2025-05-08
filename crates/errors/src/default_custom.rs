use pinocchio::program_error::{ProgramError, ToStr};

pub struct CustomError;

impl TryFrom<u32> for CustomError {
    type Error = ProgramError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(CustomError),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl ToStr for CustomError {
    fn to_str<E>(&self) -> &'static str
    where
        E: 'static + ToStr + TryFrom<u32>,
    {
        "Error: Custom error"
    }
}

impl From<CustomError> for u32 {
    fn from(_: CustomError) -> Self {
        200
    }
}

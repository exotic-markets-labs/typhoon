use typhoon::prelude::*;

pub enum SeedsError {
    InvalidOwner = 200,
}

impl TryFrom<u32> for SeedsError {
    type Error = ProgramError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(SeedsError::InvalidOwner),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl ToStr for SeedsError {
    fn to_str<E>(&self) -> &'static str
    where
        E: 'static + ToStr + TryFrom<u32>,
    {
        match self {
            SeedsError::InvalidOwner => "Error: Invalid owner",
        }
    }
}

impl From<SeedsError> for Error {
    fn from(value: SeedsError) -> Self {
        Error::new(ProgramError::Custom(value as u32))
    }
}

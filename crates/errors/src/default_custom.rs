use {
    num_traits::{FromPrimitive, ToPrimitive},
    std::fmt::Display,
};

pub struct CustomError;

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl FromPrimitive for CustomError {
    fn from_i64(_n: i64) -> Option<Self> {
        None
    }

    fn from_u64(_n: u64) -> Option<Self> {
        None
    }
}

impl ToPrimitive for CustomError {
    fn to_i64(&self) -> Option<i64> {
        None
    }

    fn to_u64(&self) -> Option<u64> {
        None
    }
}

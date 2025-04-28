use {
    crate::Error,
    num_traits::{FromPrimitive, ToPrimitive},
    std::fmt::Display,
};

pub trait ResultExtension {
    fn trace_account(self, name: impl ToString) -> Self;
}

impl<T, E> ResultExtension for Result<T, Error<E>>
where
    E: Display + FromPrimitive + ToPrimitive,
{
    fn trace_account(self, name: impl ToString) -> Self {
        self.map_err(|err| err.with_account(name))
    }
}

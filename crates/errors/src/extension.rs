use crate::Error;

pub trait ResultExtension {
    fn trace_account(self, name: &'static str) -> Self;
}

impl<T, E> ResultExtension for Result<T, Error<E>> {
    fn trace_account(self, name: &'static str) -> Self {
        self.map_err(|err| err.with_account(name))
    }
}

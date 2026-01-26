use crate::Error;

pub trait ResultExtension {
    fn trace_account(self, name: &'static str) -> Self;
}

impl<T> ResultExtension for Result<T, Error> {
    fn trace_account(self, name: &'static str) -> Self {
        self.map_err(|err| err.with_account(name))
    }
}

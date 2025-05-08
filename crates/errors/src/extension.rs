use {crate::Error, pinocchio::program_error::ToStr};

pub trait ResultExtension {
    fn trace_account(self, name: impl ToString) -> Self;
}

impl<T, E> ResultExtension for Result<T, Error<E>>
where
    E: 'static + ToStr + TryFrom<u32>,
{
    fn trace_account(self, name: impl ToString) -> Self {
        self.map_err(|err| err.with_account(name))
    }
}

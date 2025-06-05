use {
    crate::HandlerContext,
    bytemuck::{try_from_bytes, AnyBitPattern},
    core::ops::Deref,
    pinocchio::{account_info::AccountInfo, program_error::ProgramError},
    typhoon_errors::Error,
};

#[derive(Debug)]
pub struct Arg<'a, T>(&'a T);

impl<'a, T> Arg<'a, T> {
    pub fn new(arg: &'a T) -> Self {
        Arg(arg)
    }
}

impl<T> Deref for Arg<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T> HandlerContext<'a> for Arg<'a, T>
where
    T: AnyBitPattern,
{
    fn from_entrypoint(
        _accounts: &mut &'a [AccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self, Error> {
        let (arg_data, remaining) = instruction_data.split_at(core::mem::size_of::<T>());

        let arg: &T = try_from_bytes(arg_data).map_err(|_| ProgramError::InvalidInstructionData)?;

        *instruction_data = remaining;

        Ok(Arg::new(arg))
    }
}

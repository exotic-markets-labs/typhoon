use {
    crate::HandlerContext,
    bytemuck::{try_from_bytes, Pod},
    pinocchio::{account_info::AccountInfo, program_error::ProgramError},
    std::ops::Deref,
};

#[derive(Debug)]
pub struct Args<'a, T>(&'a T);

impl<'a, T> Args<'a, T> {
    pub fn new(arg: &'a T) -> Self {
        Args(arg)
    }
}

impl<T> Deref for Args<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T> HandlerContext<'a> for Args<'a, T>
where
    T: Pod,
{
    fn from_entrypoint(
        _accounts: &mut &'a [AccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self, ProgramError> {
        let arg: &T = try_from_bytes(&instruction_data[..std::mem::size_of::<T>()])
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let (_, remaining) = instruction_data.split_at(std::mem::size_of::<T>());

        *instruction_data = remaining;

        Ok(Args::new(arg))
    }
}

use {
    crate::HandlerContext,
    aligned::{Aligned, A8},
    bytemuck::Pod,
    crayfish_program::{bytes::try_from_bytes, program_error::ProgramError, RawAccountInfo},
    std::ops::Deref,
};

pub struct Args<'a, T>(&'a T);

impl<'a, T> Args<'a, T> {
    pub fn new(arg: &'a T) -> Self {
        Args(arg)
    }
}

impl<'a, T> Deref for Args<'a, T> {
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
        _accounts: &mut &'a [RawAccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self, ProgramError> {
        let arg: &T =
            try_from_bytes(instruction_data).ok_or(ProgramError::InvalidInstructionData)?;

        let (_, remaining) = instruction_data.split_at(std::mem::size_of::<Aligned<A8, T>>());
        *instruction_data = remaining;

        Ok(Args::new(arg))
    }
}

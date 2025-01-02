use {
    crate::HandlerContext,
    std::ops::Deref,
    typhoon_program::{program_error::ProgramError, RawAccountInfo},
    zerocopy::{Immutable, KnownLayout, TryFromBytes},
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
    T: KnownLayout + Immutable + TryFromBytes,
{
    fn from_entrypoint(
        _accounts: &mut &'a [RawAccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self, ProgramError> {
        let (arg, remaining) = T::try_ref_from_prefix(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        *instruction_data = remaining;

        Ok(Args::new(arg))
    }
}

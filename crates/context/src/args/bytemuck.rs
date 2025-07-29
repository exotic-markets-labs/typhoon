use {
    crate::HandlerContext,
    bytemuck::{try_from_bytes, AnyBitPattern},
    pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey},
    typhoon_errors::Error,
};

#[derive(Debug)]
pub struct Arg<'a, T>(pub &'a T);

impl<'c, T> HandlerContext<'_, '_, 'c> for Arg<'c, T>
where
    T: AnyBitPattern,
{
    #[inline(always)]
    fn from_entrypoint(
        _program_id: &Pubkey,
        _accounts: &mut &[AccountInfo],
        instruction_data: &mut &'c [u8],
    ) -> Result<Self, Error> {
        let (arg_data, remaining) = instruction_data.split_at(core::mem::size_of::<T>());

        let arg: &T = try_from_bytes(arg_data).map_err(|_| ProgramError::InvalidInstructionData)?;

        *instruction_data = remaining;

        Ok(Arg(arg))
    }
}

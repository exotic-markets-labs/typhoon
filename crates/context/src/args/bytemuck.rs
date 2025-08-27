use {
    crate::HandlerContext,
    bytemuck::AnyBitPattern,
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
        let len = core::mem::size_of::<T>();

        if len > instruction_data.len() {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        // SAFETY: The invariant `len <= instruction_data.len()` is upheld by the preceding
        // bounds check, ensuring that the split index is within the valid range [0, len]
        // where len does not exceed the slice length, thus satisfying the preconditions
        // for `split_at_unchecked`.
        let (arg_data, remaining) = unsafe { instruction_data.split_at_unchecked(len) };
        let data_ptr = arg_data.as_ptr();

        if data_ptr.align_offset(core::mem::align_of::<T>()) != 0 {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        let arg: &T = unsafe { &*(data_ptr as *const T) };

        *instruction_data = remaining;

        Ok(Arg(arg))
    }
}

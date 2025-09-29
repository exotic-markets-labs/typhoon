use {
    crate::{Context, HandlerContext},
    core::mem::MaybeUninit,
};

/// An extractor to handle fixed array contexts.
///
/// This struct allows you to deserialize arrays of context types from program
/// entrypoint. Each element in the array is deserialized using the same
/// `HandlerContext::from_entrypoint` method, consuming accounts and instruction data
/// in sequence.
///
/// # Type Parameters
/// - `T`: The context type
/// - `N`: The compile-time constant size of the array
pub struct Array<T, const N: usize>(pub [T; N]);

impl<'a, 'b, 'c, T, const N: usize> HandlerContext<'a, 'b, 'c> for Array<T, N>
where
    T: HandlerContext<'a, 'b, 'c> + Context,
{
    #[inline(always)]
    fn from_entrypoint(
        program_id: &'a pinocchio::pubkey::Pubkey,
        accounts: &mut &'b [pinocchio::account_info::AccountInfo],
        instruction_data: &mut &'c [u8],
    ) -> Result<Self, typhoon_errors::Error> {
        let mut result = [const { MaybeUninit::uninit() }; N];

        for r in result.iter_mut() {
            r.write(T::from_entrypoint(program_id, accounts, instruction_data)?);
        }

        // SAFETY: All elements have been initialized by the loop above
        let array = unsafe { result.map(|item| item.assume_init()) };
        Ok(Array(array))
    }
}

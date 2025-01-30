use {
    crate::{FromAccountInfo, ReadableAccount},
    typhoon_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo},
};

impl<'a, T> FromAccountInfo<'a> for Option<T>
where
    T: FromAccountInfo<'a> + ReadableAccount,
{
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.key() == &Pubkey::default() {
            Ok(None)
        } else {
            T::try_from_info(info).map(Some)
        }
    }
}

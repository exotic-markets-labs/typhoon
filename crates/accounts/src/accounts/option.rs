use {
    crate::{FromAccountInfo, ReadableAccount},
    pinocchio::{account_info::AccountInfo, pubkey::Pubkey},
    typhoon_errors::Error,
};

impl<'a, T> FromAccountInfo<'a> for Option<T>
where
    T: FromAccountInfo<'a> + ReadableAccount,
{
    fn try_from_info(info: &'a AccountInfo) -> Result<Self, Error> {
        if info.key() == &Pubkey::default() {
            Ok(None)
        } else {
            T::try_from_info(info).map(Some)
        }
    }
}

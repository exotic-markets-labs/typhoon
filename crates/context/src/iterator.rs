use {
    crate::HandlerContext,
    core::marker::PhantomData,
    paste::paste,
    pinocchio::{account_info::AccountInfo, program_error::ProgramError},
    typhoon_accounts::FromAccountInfo,
    typhoon_errors::Error,
};

trait FromInfos<'a>: Sized {
    fn from_infos(accounts: &mut &'a [AccountInfo]) -> Result<Self, Error>;
}

macro_rules! impl_from_infos {
    ($($t:ident),+) => {
        impl<'a, $($t),+> FromInfos<'a> for ($($t),+) where $($t: FromAccountInfo<'a>),+ {
            fn from_infos(accounts: &mut &'a [AccountInfo]) -> Result<Self, Error> {
                paste! {
                    let [$( [<acc_ $t:lower>], )+ rem @ ..] = *accounts else {
                        return Err(Error::new(ProgramError::NotEnoughAccountKeys));
                    };
                }

                paste! {
                    $( let [<val_ $t:lower>] = $t::try_from_info([<acc_ $t:lower>])?; )+

                    *accounts = rem;

                    Ok(( $( [<val_ $t:lower>] ),+ ))
                }
            }
        }
    };
}

impl_from_infos!(T1, T2);
impl_from_infos!(T1, T2, T3);
impl_from_infos!(T1, T2, T3, T4);
impl_from_infos!(T1, T2, T3, T4, T5);

impl<'a, T: FromAccountInfo<'a>> FromInfos<'a> for (T,) {
    fn from_infos(accounts: &mut &'a [AccountInfo]) -> Result<Self, Error> {
        let [acc, rem @ ..] = *accounts else {
            return Err(Error::new(ProgramError::NotEnoughAccountKeys));
        };

        let acc = T::try_from_info(acc)?;
        *accounts = rem;

        Ok((acc,))
    }
}

/// An iterator over account infos, yielding tuples of type `T` that can be constructed from
/// the current slice of accounts. The iterator advances by consuming the accounts as each item is produced.
pub struct AccountIter<'a, T> {
    accounts: &'a [AccountInfo],
    _phantom: PhantomData<T>,
}

impl<'b, T> HandlerContext<'_, 'b, '_> for AccountIter<'b, T> {
    fn from_entrypoint(
        _program_id: &pinocchio::pubkey::Pubkey,
        accounts: &mut &'b [AccountInfo],
        _instruction_data: &mut &[u8],
    ) -> Result<Self, typhoon_errors::Error> {
        Ok(AccountIter {
            accounts,
            _phantom: PhantomData,
        })
    }
}

impl<'a, T> Iterator for AccountIter<'a, T>
where
    T: FromInfos<'a>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        T::from_infos(&mut self.accounts).ok()
    }
}

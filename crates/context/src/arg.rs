use {
    crate::HandlerContext,
    solana_account_view::AccountView,
    solana_address::Address,
    typhoon_errors::{Error, ErrorCode},
    typhoon_traits::{Accessor, BytemuckStrategy},
};

pub type ArgData<'a, T, S> = <S as Accessor<'a, T>>::Data;

pub struct Arg<'a, T, S = BytemuckStrategy>(pub ArgData<'a, T, S>)
where
    S: Accessor<'a, T>;

impl<'c, T, S> HandlerContext<'_, '_, 'c> for Arg<'c, T, S>
where
    S: Accessor<'c, T>,
{
    #[inline(always)]
    fn from_entrypoint(
        _program_id: &Address,
        _accounts: &mut &[AccountView],
        instruction_data: &mut &'c [u8],
    ) -> Result<Self, Error> {
        let (arg, used) = S::access_and_consume(instruction_data)?;
        let remaining = instruction_data
            .get(used..)
            .ok_or(ErrorCode::InvalidDataLength)?;
        *instruction_data = remaining;

        Ok(Self(arg))
    }
}

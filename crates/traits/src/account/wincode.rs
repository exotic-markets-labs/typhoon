use {
    crate::{Accessor, MutAccessor},
    core::marker::PhantomData,
    solana_program_error::ProgramError,
    wincode::{config::DefaultConfig, Deserialize, SchemaRead, ZeroCopy},
};

pub struct WincodeStrategy<const ZERO_COPY: bool, C = DefaultConfig>(PhantomData<C>);

impl<'a, T> Accessor<'a, T> for WincodeStrategy<true>
where
    T: ZeroCopy + SchemaRead<'a, DefaultConfig, Dst = T> + Sized,
{
    type Data = &'a T;

    #[inline(always)]
    fn access(data: &'a [u8]) -> Result<Self::Data, ProgramError> {
        <T as ZeroCopy>::from_bytes(data).map_err(|_| ProgramError::InvalidAccountData)
    }
}

impl<'a, T> Accessor<'a, T> for WincodeStrategy<false>
where
    T: Deserialize<'a, Dst = T> + 'a,
{
    type Data = T;

    #[inline(always)]
    fn access(data: &'a [u8]) -> Result<Self::Data, ProgramError> {
        T::deserialize(data).map_err(|_| ProgramError::BorshIoError)
    }
}

impl<'a, T> MutAccessor<'a, T> for WincodeStrategy<true>
where
    T: ZeroCopy + SchemaRead<'a, DefaultConfig, Dst = T> + Sized,
{
    type Data = &'a mut T;

    #[inline(always)]
    fn access_mut(data: &'a mut [u8]) -> Result<Self::Data, ProgramError> {
        <T as ZeroCopy>::from_bytes_mut(data).map_err(|_| ProgramError::InvalidAccountData)
    }
}

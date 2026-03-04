use {
    crate::{Accessor, MutAccessor},
    core::marker::PhantomData,
    solana_program_error::ProgramError,
    wincode::{
        config::{ConfigCore, DefaultConfig},
        io::Reader,
        SchemaRead,
    },
};

pub struct WincodeStrategy<const ZERO_COPY: bool, C = DefaultConfig>(PhantomData<C>);

impl<'a, T: 'a, C> Accessor<'a, T> for WincodeStrategy<true, C>
where
    C: ConfigCore,
    &'a T: SchemaRead<'a, C, Dst = &'a T>,
{
    type Data = &'a T;

    #[inline(always)]
    fn access(data: &'a [u8]) -> Result<Self::Data, ProgramError> {
        <&T as SchemaRead<'a, C>>::get(data).map_err(|_| ProgramError::BorshIoError)
    }

    #[inline(always)]
    fn access_and_consume(data: &'a [u8]) -> Result<(Self::Data, usize), ProgramError> {
        let len = core::mem::size_of::<T>();
        let value = <&T as SchemaRead<'a, C>>::get(data).map_err(|_| ProgramError::BorshIoError)?;
        Ok((value, len))
    }
}

impl<'a, T, C> Accessor<'a, T> for WincodeStrategy<false, C>
where
    C: ConfigCore,
    T: SchemaRead<'a, C, Dst = T> + 'a,
{
    type Data = T;

    #[inline(always)]
    fn access(data: &'a [u8]) -> Result<Self::Data, ProgramError> {
        <T as SchemaRead<'a, C>>::get(data).map_err(|_| ProgramError::BorshIoError)
    }

    #[inline(always)]
    fn access_and_consume(data: &'a [u8]) -> Result<(Self::Data, usize), ProgramError> {
        let mut cursor = wincode::io::Cursor::new(data);
        let value = <T as SchemaRead<'a, C>>::get(cursor.by_ref())
            .map_err(|_| ProgramError::BorshIoError)?;
        Ok((value, cursor.position()))
    }
}

impl<'a, T: 'a, C> MutAccessor<'a, T> for WincodeStrategy<true, C>
where
    C: ConfigCore,
    &'a mut T: SchemaRead<'a, C, Dst = &'a mut T>,
{
    type Data = &'a mut T;

    #[inline(always)]
    fn access_mut(data: &'a mut [u8]) -> Result<Self::Data, ProgramError> {
        <&mut T as SchemaRead<'a, C>>::get(data).map_err(|_| ProgramError::InvalidAccountData)
    }
}

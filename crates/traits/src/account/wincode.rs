use {
    crate::{Accessor, MutAccessor, Write},
    core::marker::PhantomData,
    solana_program_error::ProgramError,
    wincode::{
        config::{ConfigCore, Configuration, DefaultConfig, DEFAULT_PREALLOCATION_SIZE_LIMIT},
        len::UseIntLen,
        SchemaRead, SchemaWrite,
    },
};

pub type BorshConfig = Configuration<true, DEFAULT_PREALLOCATION_SIZE_LIMIT, UseIntLen<u32>>;

pub type BorshStrategy<const ZERO_COPY: bool> = WincodeStrategy<ZERO_COPY, BorshConfig>;

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
    fn read(data: &mut &'a [u8]) -> Result<Self::Data, ProgramError> {
        <&T as SchemaRead<'a, C>>::get(data).map_err(|_| ProgramError::BorshIoError)
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
    fn read(data: &mut &'a [u8]) -> Result<Self::Data, ProgramError> {
        <T as SchemaRead<'a, C>>::get(data).map_err(|_| ProgramError::BorshIoError)
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

impl<T, const ZERO_COPY: bool, C> Write<T> for WincodeStrategy<ZERO_COPY, C>
where
    C: ConfigCore,
    T: SchemaWrite<C, Src = T>,
{
    #[inline(always)]
    fn write_into(writer: impl wincode::io::Writer, data: &T) -> Result<(), ProgramError> {
        T::write(writer, data).map_err(|_| ProgramError::BorshIoError)
    }

    #[inline(always)]
    fn size(data: &T) -> Result<usize, ProgramError> {
        T::size_of(data).map_err(|_| ProgramError::BorshIoError)
    }
}

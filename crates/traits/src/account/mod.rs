#[cfg(feature = "bytemuck")]
mod bytemuck;
#[cfg(feature = "wincode")]
mod wincode;

use ::wincode::io::Writer;
#[cfg(feature = "bytemuck")]
pub use bytemuck::*;
use solana_program_error::ProgramError;
#[cfg(feature = "wincode")]
pub use wincode::*;

pub trait Accessor<'a, T> {
    type Data: 'a;

    fn access(data: &'a [u8]) -> Result<Self::Data, ProgramError>;

    fn read(data: &mut &'a [u8]) -> Result<Self::Data, ProgramError>;
}

pub trait MutAccessor<'a, T> {
    type Data: 'a;

    fn access_mut(data: &'a mut [u8]) -> Result<Self::Data, ProgramError>;
}

pub trait Write<T> {
    fn size(data: &T) -> Result<usize, ProgramError>;
    fn write_into(writer: impl Writer, data: &T) -> Result<(), ProgramError>;
}

pub trait DataStrategy {
    type Strategy;
}

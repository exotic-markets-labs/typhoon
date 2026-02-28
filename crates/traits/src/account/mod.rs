#[cfg(feature = "borsh")]
mod borsh;
#[cfg(feature = "bytemuck")]
mod bytemuck;
#[cfg(feature = "wincode")]
mod wincode;

#[cfg(feature = "borsh")]
pub use borsh::*;
#[cfg(feature = "bytemuck")]
pub use bytemuck::*;
use solana_program_error::ProgramError;
#[cfg(feature = "wincode")]
pub use wincode::*;

pub trait Accessor<'a, T> {
    type Data: 'a;

    fn access(data: &'a [u8]) -> Result<Self::Data, ProgramError>;
}

pub trait MutAccessor<'a, T> {
    type Data: 'a;

    fn access_mut(data: &'a mut [u8]) -> Result<Self::Data, ProgramError>;
}

pub trait AccountStrategy {
    type Strategy;
}

#[cfg(feature = "borsh")]
mod borsh;

#[cfg(feature = "borsh")]
pub use borsh::*;
use core::mem::MaybeUninit;

pub const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

#[inline(always)]
pub fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}

mod writer;

pub use writer::*;
use {core::mem::MaybeUninit, pinocchio::instruction::Seed};

pub const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();
pub const UNINIT_SEED: MaybeUninit<Seed> = MaybeUninit::<Seed>::uninit();

#[inline(always)]
pub fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}

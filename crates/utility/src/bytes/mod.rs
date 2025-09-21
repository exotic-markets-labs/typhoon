mod writer;

pub use writer::*;
use {
    core::mem::MaybeUninit,
    pinocchio::{
        account_info::AccountInfo,
        instruction::{AccountMeta, Seed},
    },
};

pub const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();
pub const UNINIT_SEED: MaybeUninit<Seed> = MaybeUninit::<Seed>::uninit();
pub const UNINIT_META: MaybeUninit<AccountMeta> = MaybeUninit::<AccountMeta>::uninit();
pub const UNINIT_INFO: MaybeUninit<&AccountInfo> = MaybeUninit::<&AccountInfo>::uninit();

#[inline(always)]
pub fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}

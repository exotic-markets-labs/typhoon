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
    let len = destination.len().min(source.len());
    unsafe {
        core::ptr::copy_nonoverlapping(source.as_ptr(), destination.as_mut_ptr() as *mut u8, len);
    }
}

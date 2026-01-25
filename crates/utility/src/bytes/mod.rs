mod writer;

pub use writer::*;
use {
    core::mem::MaybeUninit,
    pinocchio::{cpi::Seed, instruction::InstructionAccount, AccountView},
};

pub const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();
pub const UNINIT_SEED: MaybeUninit<Seed> = MaybeUninit::<Seed>::uninit();
pub const UNINIT_INS_ACC: MaybeUninit<InstructionAccount> =
    MaybeUninit::<InstructionAccount>::uninit();
pub const UNINIT_ACC_VIEW: MaybeUninit<&AccountView> = MaybeUninit::<&AccountView>::uninit();

#[inline(always)]
pub fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    let len = destination.len().min(source.len());
    unsafe {
        core::ptr::copy_nonoverlapping(source.as_ptr(), destination.as_mut_ptr() as *mut u8, len);
    }
}

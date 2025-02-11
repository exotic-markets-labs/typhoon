#[cfg(feature = "nostd")]
mod vanilla;

#[cfg(feature = "nostd")]
pub use vanilla::*;

#[cfg(feature = "pinocchio")]
mod pinocchio;

#[cfg(feature = "pinocchio")]
pub use pinocchio::*;

pub mod bytes;

#[cfg(any(feature = "pinocchio", feature = "nostd"))]
pub trait ToMeta {
    fn to_meta(&self, is_writable: bool, is_signer: bool) -> AccountMeta;
}

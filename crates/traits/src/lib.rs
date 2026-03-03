//! Traits for the Typhoon protocol.
//!
//! This crate contains common traits used in the Typhoon ecosystem.

#![no_std]

mod account;

pub use account::*;
use solana_address::Address;

/// Trait to check whether a program ID matches an expected program.
pub trait CheckProgramId {
    /// Returns `true` if the given program ID matches this program.
    fn address_eq(program_id: &Address) -> bool;
}

/// Trait to check whether a program ID is a valid owner.
pub trait CheckOwner {
    /// Returns `true` if the given program ID is an allowed owner.
    fn owned_by(program_id: &Address) -> bool;
}

/// Trait to define the unique discriminator for an account.
pub trait Discriminator {
    /// The discriminator bytes.
    const DISCRIMINATOR: &'static [u8];
}

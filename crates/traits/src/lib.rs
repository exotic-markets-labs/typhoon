//! Traits for the Typhoon protocol.
//!
//! This crate contains common traits used in the Typhoon ecosystem.

#![no_std]

use solana_address::Address;

/// Trait to define the program ID of a program.
pub trait ProgramId {
    /// The program ID.
    const ID: Address;
}

/// Trait to define multiple program IDs associated with a program.
pub trait ProgramIds {
    /// The program IDs.
    const IDS: &'static [Address];
}

/// Trait to define the owner of an account.
pub trait Owner {
    /// The owner address.
    const OWNER: Address;
}

/// Trait to define multiple possible owners for an account.
pub trait Owners {
    /// The owner addresses.
    const OWNERS: &'static [Address];
}

/// Trait to define the unique discriminator for an account.
pub trait Discriminator {
    /// The discriminator bytes.
    const DISCRIMINATOR: &'static [u8];
}

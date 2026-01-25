#![no_std]

use solana_address::Address;

pub trait ProgramId {
    const ID: Address;
}

pub trait ProgramIds {
    const IDS: &'static [Address];
}

pub trait Owner {
    const OWNER: Address;
}

pub trait Owners {
    const OWNERS: &'static [Address];
}

pub trait Discriminator {
    const DISCRIMINATOR: &'static [u8];
}

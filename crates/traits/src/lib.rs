use pinocchio::pubkey::Pubkey;

pub trait ProgramId {
    const ID: Pubkey;
}

pub trait ProgramIds {
    const IDS: &'static [Pubkey];
}

pub trait Owner {
    const OWNER: Pubkey;
}

pub trait Owners {
    const OWNERS: &'static [Pubkey];
}

pub trait Discriminator {
    const DISCRIMINATOR: &'static [u8];
}

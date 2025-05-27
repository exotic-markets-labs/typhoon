use typhoon_accounts::{Mut, ReadableAccount, UncheckedAccount};

pub trait ChecksExt: ReadableAccount {
    fn is_initialized(&self) -> bool {
        !self.is_owned_by(&pinocchio_system::ID)
    }
}

impl ChecksExt for Mut<UncheckedAccount<'_>> {}
impl ChecksExt for UncheckedAccount<'_> {}

use crate::{impl_sysvar_get, sysvars::Sysvar};
pub use solana_clock::*;

impl Sysvar for Clock {
    impl_sysvar_get!(sol_get_clock_sysvar);
}

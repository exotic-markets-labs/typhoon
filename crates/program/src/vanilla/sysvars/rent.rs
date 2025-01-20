use crate::{impl_sysvar_get, sysvars::Sysvar};
pub use solana_rent::*;

impl Sysvar for Rent {
    impl_sysvar_get!(sol_get_rent_sysvar);
}

mod invoke;

pub mod sysvars;

pub use {
    invoke::*,
    nostd_system_program as system_program,
    pubkey::declare_id,
    solana_msg::*,
    solana_nostd_entrypoint::{basic_panic_impl, entrypoint_nostd, noalloc_allocator, Ref, RefMut},
    solana_program_entrypoint::*,
    solana_program_error as program_error,
};

pub type RawAccountInfo = solana_nostd_entrypoint::NoStdAccountInfo;
pub type Account = solana_nostd_entrypoint::AccountInfoC;
pub type SignerSeeds<'a, 'b> = &'a [&'b [u8]];
pub type SignerSeed<'a> = &'a [u8];
pub type AccountMeta = solana_nostd_entrypoint::AccountMetaC;

pub mod definitions {
    #[macro_export]
    macro_rules! define_syscall {
        (fn $name:ident($($arg:ident: $typ:ty),*) -> $ret:ty, $code:ident) => {
            extern "C" {
                pub fn $name($($arg: $typ),*) -> $ret;
            }
        };
        (fn $name:ident($($arg:ident: $typ:ty),*), $code:ident) => {
            define_syscall!(fn $name($($arg: $typ),*) -> (), $code);
        }
    }

    pub const SOL_GET_CLOCK_SYSVAR: u32 = 36;
    pub const SOL_GET_RENT_SYSVAR: u32 = 41;

    define_syscall!(fn sol_get_clock_sysvar(addr: *mut u8) -> u64, SOL_GET_CLOCK_SYSVAR);
    define_syscall!(fn sol_get_rent_sysvar(addr: *mut u8) -> u64, SOL_GET_RENT_SYSVAR);
}

#[macro_export]
macro_rules! program_entrypoint {
    ($name: ident) => {
        use solana_nostd_entrypoint::NoStdAccountInfo;

        $crate::entrypoint_nostd!(process_instruction, 32);
        // $crate::noalloc_allocator!();
        $crate::basic_panic_impl!();
    };
}

impl crate::ToMeta for RawAccountInfo {
    fn to_meta(&self, is_writable: bool, is_signer: bool) -> AccountMeta {
        let mut meta = if is_signer {
            self.to_meta_c_signer()
        } else {
            self.to_meta_c()
        };

        meta.is_writable = is_writable;
        meta
    }
}

pub const fn pubkey_from_array(pubkey_array: [u8; 32]) -> pubkey::Pubkey {
    pubkey::Pubkey::new_from_array(pubkey_array)
}

pub mod pubkey {
    pub use solana_pubkey::*;

    pub fn find_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(seeds, program_id)
    }

    pub fn try_find_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> Option<(Pubkey, u8)> {
        Pubkey::try_find_program_address(seeds, program_id)
    }

    pub fn create_program_address(
        seeds: &[&[u8]],
        program_id: &Pubkey,
    ) -> Result<Pubkey, solana_program_error::ProgramError> {
        Ok(Pubkey::create_program_address(seeds, program_id)?)
    }
}

#[macro_export]
macro_rules! seeds {
    ($($seed:expr),*) => {
        [$(
            $seed
        ),*]
    };
}

pub use {
    pinocchio::{
        account_info::{self, Ref, RefMut},
        entrypoint,
        instruction::{self, AccountMeta, Instruction},
        msg as log,
        program::{self, invoke, invoke_signed},
        program_error, pubkey, sysvars, ProgramResult,
    },
    pinocchio_log,
    pinocchio_pubkey::declare_id,
    pinocchio_system as system_program,
};

pub type RawAccountInfo = account_info::AccountInfo;
pub type SignerSeeds<'a, 'b> = instruction::Signer<'a, 'b>;
pub type SignerSeed<'a> = instruction::Seed<'a>;

#[macro_export]
macro_rules! program_entrypoint {
    ($name: ident) => {
        use typhoon_program::entrypoint;

        $crate::entrypoint!(process_instruction);
    };
}

impl crate::ToMeta for RawAccountInfo {
    fn to_meta(&self, is_writable: bool, is_signer: bool) -> AccountMeta {
        AccountMeta::new(self.key(), is_writable, is_signer)
    }
}

pub const fn pubkey_from_array(pubkey_array: [u8; 32]) -> pubkey::Pubkey {
    pubkey_array
}

#[macro_export]
macro_rules! msg {
    ($msg:expr) => {
        typhoon_program::log!($msg);
    };
    ($($arg:tt)*) => {{
        use typhoon_program::pinocchio_log;

        typhoon_program::pinocchio_log::log!($($arg)*);
    }};
}

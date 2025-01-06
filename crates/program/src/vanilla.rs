pub use {
    nostd_entrypoint_invoke::invoke_unchecked,
    nostd_system_program as system_program,
    solana_nostd_entrypoint::{
        basic_panic_impl, entrypoint_nostd, noalloc_allocator,
        solana_program::{
            entrypoint::{ProgramResult, SUCCESS},
            *,
        },
        Ref, RefMut,
    },
};
use {program_error::ProgramError, solana_nostd_entrypoint::InstructionC, std::mem::MaybeUninit};

pub mod sysvars {
    pub use solana_nostd_entrypoint::solana_program::sysvar::*;
}

pub type RawAccountInfo = solana_nostd_entrypoint::NoStdAccountInfo;
pub type Account = solana_nostd_entrypoint::AccountInfoC;
pub type SignerSeeds<'a, 'b> = &'a [&'b [u8]];
pub type SignerSeed<'a> = &'a [u8];
pub type AccountMeta = solana_nostd_entrypoint::AccountMetaC;

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

#[repr(C)]
pub struct Instruction<'a, 'b, 'c> {
    /// Public key of the program.
    pub program_id: &'a pubkey::Pubkey,

    /// Data expected by the program instruction.
    pub data: &'b [u8],

    /// Metadata describing accounts that should be passed to the program.
    pub accounts: &'c [AccountMeta],
}

impl From<&Instruction<'_, '_, '_>> for InstructionC {
    fn from(value: &Instruction) -> Self {
        InstructionC {
            program_id: value.program_id,
            accounts: value.accounts.as_ptr(),
            accounts_len: value.accounts.len() as u64,
            data: value.data.as_ptr(),
            data_len: value.data.len() as u64,
        }
    }
}

/// Invoke a cross-program instruction.
///
/// # Important
///
/// The accounts on the `account_infos` slice must be in the same order as the
/// `accounts` field of the `instruction`.
#[inline(always)]
pub fn invoke<const ACCOUNTS: usize>(
    instruction: &Instruction,
    account_infos: &[&RawAccountInfo; ACCOUNTS],
) -> ProgramResult {
    invoke_signed(instruction, account_infos, &[])
}

/// Invoke a cross-program instruction with signatures.
///
/// # Important
///
/// The accounts on the `account_infos` slice must be in the same order as the
/// `accounts` field of the `instruction`.
pub fn invoke_signed<const ACCOUNTS: usize>(
    instruction: &Instruction,
    account_infos: &[&RawAccountInfo; ACCOUNTS],
    signers_seeds: &[SignerSeeds],
) -> ProgramResult {
    if instruction.accounts.len() < ACCOUNTS {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    const UNINIT: MaybeUninit<Account> = MaybeUninit::<Account>::uninit();
    let mut accounts = [UNINIT; ACCOUNTS];

    for index in 0..ACCOUNTS {
        let account_info = account_infos[index];
        let account_meta = &instruction.accounts[index];

        if *account_info.key() != unsafe { *account_meta.pubkey } {
            return Err(ProgramError::InvalidArgument);
        }

        if account_meta.is_writable {
            let _ = account_info.try_borrow_mut_data()?;
            let _ = account_info.try_borrow_mut_lamports()?;
        } else {
            let _ = account_info.try_borrow_data()?;
            let _ = account_info.try_borrow_lamports()?;
        }

        accounts[index].write(Account::from(account_infos[index].to_info_c()));
    }

    invoke_unchecked(&InstructionC::from(instruction), accounts, signers_seeds)?;

    Ok(())
}

pub const fn pubkey_from_array(pubkey_array: [u8; 32]) -> pubkey::Pubkey {
    pubkey::Pubkey::new_from_array(pubkey_array)
}

pub mod pubkey {
    pub use solana_nostd_entrypoint::solana_program::pubkey::*;

    pub fn find_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(seeds, program_id)
    }

    pub fn try_find_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> Option<(Pubkey, u8)> {
        Pubkey::try_find_program_address(seeds, program_id)
    }

    pub fn create_program_address(
        seeds: &[&[u8]],
        program_id: &Pubkey,
    ) -> Result<Pubkey, super::ProgramError> {
        Ok(Pubkey::create_program_address(seeds, program_id)?)
    }
}

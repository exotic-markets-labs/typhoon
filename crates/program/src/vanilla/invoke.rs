use {
    core::mem::MaybeUninit,
    solana_define_syscall::define_syscall,
    solana_nostd_entrypoint::{AccountInfoC, InstructionC, NoStdAccountInfo},
    solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError,
};

define_syscall!(fn sol_invoke_signed_c(instruction_addr: *const u8, account_infos_addr: *const u8, account_infos_len: u64, signers_seeds_addr: *const u8, signers_seeds_len: u64) -> u64);

#[repr(C)]
pub struct Instruction<'a, 'b, 'c> {
    /// Public key of the program.
    pub program_id: &'a solana_pubkey::Pubkey,

    /// Data expected by the program instruction.
    pub data: &'b [u8],

    /// Metadata describing accounts that should be passed to the program.
    pub accounts: &'c [crate::AccountMeta],
}

impl From<&Instruction<'_, '_, '_>> for InstructionC {
    fn from(value: &Instruction) -> Self {
        InstructionC {
            program_id: value.program_id as *const crate::pubkey::Pubkey,
            accounts: value.accounts.as_ptr(),
            accounts_len: value.accounts.len() as u64,
            data: value.data.as_ptr(),
            data_len: value.data.len() as u64,
        }
    }
}

/// Invoke a cross-program instruction with signatures but don't enforce Rust's
/// aliasing rules.
///
/// This function is like [`invoke_signed`] except that it does not check that
/// [`RefCell`]s within [`AccountInfo`]s are properly borrowable as described in
/// the documentation for that function. Those checks consume CPU cycles that
/// this function avoids.
///
/// [`RefCell`]: std::cell::RefCell
///
/// # Safety
///
/// __This function is incorrectly missing an `unsafe` declaration.__
///
/// If any of the writable accounts passed to the callee contain data that is
/// borrowed within the calling program, and that data is written to by the
/// callee, then Rust's aliasing rules will be violated and cause undefined
/// behavior.
pub fn invoke_unchecked<const ACCOUNTS: usize>(
    instruction: &InstructionC,
    account_infos: [MaybeUninit<AccountInfoC>; ACCOUNTS],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    #[cfg(target_os = "solana")]
    unsafe {
        sol_invoke_signed_c(
            instruction as *const InstructionC as *const u8,
            account_infos.as_ptr() as *const u8,
            account_infos.len() as u64,
            signers_seeds.as_ptr() as *const u8,
            signers_seeds.len() as u64,
        );
    }

    #[cfg(not(target_os = "solana"))]
    core::hint::black_box(&(&instruction, &account_infos, &signers_seeds));

    Ok(())
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
    account_infos: &[&NoStdAccountInfo; ACCOUNTS],
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
    account_infos: &[&NoStdAccountInfo; ACCOUNTS],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    if instruction.accounts.len() < ACCOUNTS {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    const UNINIT: MaybeUninit<AccountInfoC> = MaybeUninit::<AccountInfoC>::uninit();
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

        accounts[index].write(account_infos[index].to_info_c());
    }

    invoke_unchecked(&InstructionC::from(instruction), accounts, signers_seeds)?;

    Ok(())
}

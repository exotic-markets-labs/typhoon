#![no_std]

#[cfg(all(feature = "cpi", not(feature = "client")))]
use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
};
pub use typhoon_to_metas_macro::ToMetas;
use {
    bytemuck::{bytes_of, NoUninit},
    core::mem::MaybeUninit,
};
#[cfg(all(feature = "client", not(feature = "cpi")))]
use {
    solana_instruction::{AccountMeta, Instruction},
    solana_pubkey::Pubkey,
};

pub trait ToMetas<const ACCOUNTS: usize> {
    fn to_metas() -> [(bool, bool, bool); ACCOUNTS];
}

pub struct InstructionHelper<const A: usize, const D: usize> {
    metas_len: usize,
    metas_buffer: [MaybeUninit<(bool, bool, bool)>; A],
    data_len: usize,
    data_buffer: [MaybeUninit<u8>; D],
}

impl<const A: usize, const D: usize> Default for InstructionHelper<A, D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const A: usize, const D: usize> InstructionHelper<A, D> {
    const ELEM_META: MaybeUninit<(bool, bool, bool)> = MaybeUninit::uninit();
    const INIT_METAS: [MaybeUninit<(bool, bool, bool)>; A] = [Self::ELEM_META; A];
    const ELEM_DATA: MaybeUninit<u8> = MaybeUninit::uninit();
    const INIT_DATA: [MaybeUninit<u8>; D] = [Self::ELEM_DATA; D];

    pub const fn new() -> Self {
        Self {
            metas_len: 0,
            metas_buffer: Self::INIT_METAS,
            data_len: 0,
            data_buffer: Self::INIT_DATA,
        }
    }

    pub fn add_context<const ACCOUNTS: usize, T: ToMetas<ACCOUNTS>>(&mut self) {
        let len = core::cmp::min(self.metas_len + ACCOUNTS, A);
        for (d, s) in self.metas_buffer[self.metas_len..len]
            .iter_mut()
            .zip(T::to_metas().iter())
        {
            d.write(*s);
        }
        self.metas_len = len;
    }

    pub fn extend_bytemuck_data<T: NoUninit>(&mut self, data: &T) {
        let bytes = bytes_of(data);
        let len = core::cmp::min(self.data_len + bytes.len(), D);
        for (d, s) in self.data_buffer[self.data_len..len]
            .iter_mut()
            .zip(bytes.iter())
        {
            d.write(*s);
        }
        self.data_len = len;
    }

    #[cfg(all(feature = "client", not(feature = "cpi")))]
    pub fn to_instruction(&self, program_id: Pubkey, keys: &[Pubkey; A]) -> Instruction {
        let mut accounts = Vec::with_capacity(A);

        for i in 0..self.metas_len {
            let (is_optional, is_writable, is_signer) =
                unsafe { self.metas_buffer[i].assume_init() };

            if keys[i] == Pubkey::default() && is_optional {
                accounts[i] = AccountMeta::new(keys[i], false);
            } else if is_writable {
                accounts[i] = AccountMeta::new(keys[i], is_signer)
            } else {
                accounts[i] = AccountMeta::new_readonly(keys[i], is_signer)
            }
        }

        Instruction {
            program_id,
            accounts,
            data: unsafe { Vec::from_raw_parts(self.data_buffer.as_ptr() as _, self.data_len, D) },
        }
    }

    #[cfg(all(feature = "cpi", not(feature = "client")))]
    pub fn call_cpi(
        &self,
        program_id: &Pubkey,
        accounts: &[&AccountInfo; A],
        seeds: &[Signer],
    ) -> Result<(), ProgramError> {
        if self.metas_len < A {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let mut metas: [MaybeUninit<AccountMeta>; A] = [const { MaybeUninit::uninit() }; A];
        for i in 0..self.metas_len {
            let (is_optional, is_writable, is_signer) =
                unsafe { self.metas_buffer[i].assume_init() };

            let acc_key = accounts[i].key();
            if is_optional && acc_key.is_empty() {
                metas[i].write(AccountMeta::new(acc_key, false, false));
            } else {
                metas[i].write(AccountMeta::new(acc_key, is_writable, is_signer));
            }
        }

        let ix = Instruction {
            program_id,
            data: unsafe {
                core::slice::from_raw_parts(self.data_buffer.as_ptr() as _, self.data_len)
            },
            accounts: unsafe { core::slice::from_raw_parts(metas.as_ptr() as _, A) },
        };

        invoke_signed(&ix, accounts, seeds)
    }
}

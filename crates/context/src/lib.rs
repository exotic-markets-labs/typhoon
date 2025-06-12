#![no_std]

use {bytemuck::NoUninit, paste::paste};

mod args;
mod remaining_accounts;

pub use args::*;
use {
    pinocchio::{account_info::AccountInfo, cpi::set_return_data, pubkey::Pubkey},
    typhoon_errors::Error,
};

pub trait HandlerContext<'a>: Sized {
    fn from_entrypoint(
        program_id: &Pubkey,
        accounts: &mut &'a [AccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self, Error>;
}

pub trait Handler<'a, T> {
    type Output: NoUninit;

    fn call(
        self,
        program_id: &Pubkey,
        accounts: &mut &'a [AccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self::Output, Error>;
}

impl<'a, F, O> Handler<'a, ()> for F
where
    F: FnOnce() -> Result<O, Error>,
    O: NoUninit,
{
    type Output = O;

    fn call(
        self,
        _program_id: &Pubkey,
        _accounts: &mut &'a [AccountInfo],
        _instruction_data: &mut &'a [u8],
    ) -> Result<Self::Output, Error> {
        (self)()
    }
}

macro_rules! impl_handler {
    ($( $t:ident ),+) => {
        impl<'a, $( $t, )* F, O> Handler<'a, ($( $t, )*)> for F
        where
            F: FnOnce($( $t ),*) -> Result<O, Error>,
            O: NoUninit,
            $(
                $t: HandlerContext<'a>,
            )*
        {
            type Output = O;

            fn call(
                self,
                program_id: &Pubkey,
                accounts: &mut &'a [AccountInfo],
                instruction_data: &mut &'a [u8],
            ) -> Result<Self::Output, Error> {
                paste! {
                    $(
                        let [<$t:lower>] = $t::from_entrypoint(program_id, accounts, instruction_data)?;
                    )*
                    (self)($( [<$t:lower>], )*)
                }
            }
        }
    };
}

impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);
impl_handler!(T1, T2, T3, T4);
impl_handler!(T1, T2, T3, T4, T5);
impl_handler!(T1, T2, T3, T4, T5, T6);
impl_handler!(T1, T2, T3, T4, T5, T6, T7);

pub fn handle<'a, T, H>(
    program_id: &Pubkey,
    mut accounts: &'a [AccountInfo],
    mut instruction_data: &'a [u8],
    handler: H,
) -> Result<(), Error>
where
    H: Handler<'a, T>,
{
    match handler.call(program_id, &mut accounts, &mut instruction_data) {
        Ok(res) => {
            if core::mem::size_of::<H::Output>() > 0 {
                set_return_data(bytemuck::bytes_of(&res));
            }

            Ok(())
        }
        Err(err) => Err(err),
    }
}

#![no_std]

mod args;
mod array;
mod iterator;
mod program_id;
mod remaining_accounts;

pub use {args::*, array::*, iterator::*, program_id::*, remaining_accounts::*};
use {
    bytemuck::NoUninit, paste::paste, solana_account_view::AccountView, solana_address::Address,
    solana_instruction_view::cpi::set_return_data, solana_program_error::ProgramError,
    typhoon_errors::Error,
};

/// Marker trait for context types. This trait is used only for identification purposes.
pub trait Context {}

pub trait HandlerContext<'a, 'b, 'c>: Sized {
    fn from_entrypoint(
        program_id: &'a Address,
        accounts: &mut &'b [AccountView],
        instruction_data: &mut &'c [u8],
    ) -> Result<Self, Error>;
}

pub trait Handler<'a, 'b, 'c, T> {
    type Output: NoUninit;

    fn call(
        self,
        program_id: &'a Address,
        accounts: &mut &'b [AccountView],
        instruction_data: &mut &'c [u8],
    ) -> Result<Self::Output, Error>;
}

impl<F, O> Handler<'_, '_, '_, ()> for F
where
    F: FnOnce() -> Result<O, Error>,
    O: NoUninit,
{
    type Output = O;

    fn call(
        self,
        _program_id: &Address,
        _accounts: &mut &[AccountView],
        _instruction_data: &mut &[u8],
    ) -> Result<Self::Output, Error> {
        (self)()
    }
}

macro_rules! impl_handler {
    ($( $t:ident ),+) => {
        impl<'a, 'b, 'c, $( $t, )* F, O> Handler<'a, 'b, 'c, ($( $t, )*)> for F
        where
            F: FnOnce($( $t ),*) -> Result<O, Error>,
            O: NoUninit,
            $(
                $t: HandlerContext<'a, 'b, 'c>,
            )*
        {
            type Output = O;

            fn call(
                self,
                program_id: &'a Address,
                accounts: &mut &'b [AccountView],
                instruction_data: &mut &'c [u8],
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

pub fn handle<'a, 'b, 'c, T, H>(
    program_id: &'a Address,
    mut accounts: &'b [AccountView],
    mut instruction_data: &'c [u8],
    handler: H,
) -> Result<(), Error>
where
    H: Handler<'a, 'b, 'c, T>,
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

#[macro_export]
macro_rules! basic_router {
    ($($dis:literal => $fn_ident: ident),+ $(,)?) => {
        |program_id: &Address, accounts: &[AccountView], instruction_data: &[u8]| {
            let (discriminator, data) = instruction_data
                .split_first()
                .ok_or(ProgramError::InvalidInstructionData)?;

            let result = match discriminator {
                $($dis => handle(program_id, accounts, data, $fn_ident),)*
                _ => Err(ErrorCode::UnknownInstruction.into()),
            };

            #[cfg(feature = "logging")]
            result.inspect_err(|e| log_error::<LogError>(e))?;

            #[cfg(not(feature = "logging"))]
            result?;

            Ok(())
        }
    };
}

pub type EntryFn = fn(&Address, &[AccountView], &[u8]) -> Result<(), ProgramError>;

#[macro_export]
macro_rules! entrypoint {
    () => {
        $crate::entrypoint!(@inner inline(always));
    };
    (no_inline) => {
        $crate::entrypoint!(@inner inline(never));
    };
    (@inner $($inline:tt)*) => {
        program_entrypoint!(process_instruction);

        #[ $($inline)* ]
        pub fn process_instruction(
            program_id: &Address,
            accounts: &[AccountView],
            instruction_data: &[u8],
        ) -> Result<(), ProgramError> {
            ROUTER(program_id, accounts, instruction_data)
        }
    };
}

use paste::paste;

mod args;
mod remaining_accounts;

pub use args::*;
use {pinocchio::account_info::AccountInfo, typhoon_errors::Error};

pub trait HandlerContext<'a>: Sized {
    fn from_entrypoint(
        accounts: &mut &'a [AccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self, Error>;
}

pub trait Handler<'a, T> {
    type Output;

    fn call(
        self,
        accounts: &mut &'a [AccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self::Output, Error>;
}

impl<'a, F, O> Handler<'a, ()> for F
where
    F: FnOnce() -> Result<O, Error>,
{
    type Output = O;

    fn call(
        self,
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
            $(
                $t: HandlerContext<'a>,
            )*
        {
            type Output = O;

            fn call(
                self,
                accounts: &mut &'a [AccountInfo],
                instruction_data: &mut &'a [u8],
            ) -> Result<Self::Output, Error> {
                paste! {
                    $(
                        let [<$t:lower>] = $t::from_entrypoint(accounts, instruction_data)?;
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
    mut accounts: &'a [AccountInfo],
    mut instruction_data: &'a [u8],
    handler: H,
) -> Result<H::Output, Error>
where
    H: Handler<'a, T>,
{
    handler.call(&mut accounts, &mut instruction_data)
}

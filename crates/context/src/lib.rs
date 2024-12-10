use {
    crayfish_program::{program_error::ProgramError, RawAccountInfo},
    paste::paste,
};

pub mod args;
mod remaining_accounts;

pub trait HandlerContext<'a>: Sized {
    fn from_entrypoint(
        accounts: &mut &'a [RawAccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self, ProgramError>;
}

pub trait Handler<'a, T> {
    type Output;

    fn call(
        self,
        accounts: &mut &'a [RawAccountInfo],
        instruction_data: &mut &'a [u8],
    ) -> Result<Self::Output, ProgramError>;
}

impl<'a, F, O> Handler<'a, ()> for F
where
    F: FnOnce() -> Result<O, ProgramError>,
{
    type Output = O;

    fn call(
        self,
        _accounts: &mut &'a [RawAccountInfo],
        _instruction_data: &mut &'a [u8],
    ) -> Result<Self::Output, ProgramError> {
        (self)()
    }
}

macro_rules! impl_handler {
    ($( $t:ident ),+) => {
        impl<'a, $( $t, )* F, O> Handler<'a, ($( $t, )*)> for F
        where
            F: FnOnce($( $t ),*) -> Result<O, ProgramError>,
            $(
                $t: HandlerContext<'a>,
            )*
        {
            type Output = O;

            fn call(
                self,
                accounts: &mut &'a [RawAccountInfo],
                instruction_data: &mut &'a [u8],
            ) -> Result<Self::Output, ProgramError> {
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
    mut accounts: &'a [RawAccountInfo],
    mut instruction_data: &'a [u8],
    handler: H,
) -> Result<H::Output, ProgramError>
where
    H: Handler<'a, T>,
{
    handler.call(&mut accounts, &mut instruction_data)
}

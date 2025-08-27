#[repr(u8)]
#[derive(Clone, Debug)]
#[rustfmt::skip]
pub enum Instruction {
    Ping = 0,
    Log = 1,
    CreateAccount = 2,
    Transfer = 3,
    UncheckedAccount = 4,
}

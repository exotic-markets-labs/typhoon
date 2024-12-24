use {
    bytemuck::{Pod, Zeroable},
    typhoon::prelude::*,
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct InitArgs {
    pub value: u64,
}

#[context]
#[args(InitArgs)]
pub struct InitContext {
    pub payer: Signer,
    #[constraint(
        init,
        payer = payer,
        space = Buffer::SPACE
    )]
    pub buffer: Mut<Account<Buffer>>,
    pub system: Program<System>,
}

#[context]
#[args(value: u64, other_value: u64)]
pub struct SetValueContext {
    pub buffer: Mut<Account<Buffer>>,
}

handlers! {
    initialize,
    set_value,
    set_and_add_values,
}

pub fn initialize(ctx: InitContext) -> Result<(), ProgramError> {
    ctx.buffer.mut_data()?.value1 = ctx.args.value;

    Ok(())
}

pub fn set_value(ctx: SetValueContext, more_args: Args<u64>) -> Result<(), ProgramError> {
    let mut data = ctx.buffer.mut_data()?;
    data.value1 = ctx.args.value;
    data.value2 = *more_args;

    Ok(())
}

pub fn set_and_add_values(
    ctx_a: SetValueContext,
    ctx_b: SetValueContext,
) -> Result<(), ProgramError> {
    ctx_a.buffer.mut_data()?.value1 = ctx_a.args.value;
    ctx_b.buffer.mut_data()?.value1 = ctx_b.args.value;
    ctx_a.buffer.mut_data()?.value2 = ctx_a.args.value + ctx_b.args.value;

    Ok(())
}

#[account]
pub struct Buffer {
    pub value1: u64,
    pub value2: u64,
}

impl Buffer {
    const SPACE: usize = std::mem::size_of::<Buffer>();
}

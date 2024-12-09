use {
    bytemuck::{Pod, Zeroable},
    crayfish_account_macro::account,
    crayfish_accounts::{
        Account, FromAccountInfo, Mut, Program, ReadableAccount, Signer, System, WritableAccount,
    },
    crayfish_context_macro::{context, instruction},
    crayfish_handler_macro::handlers,
    crayfish_program::{program_error::ProgramError, pubkey::Pubkey},
    crayfish_program_id_macro::program_id,
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

handlers! {
    initialize,
    increment,
}

#[context]
#[instruction(admin: Pubkey, bump: u64)]
pub struct InitContext {
    pub payer: Signer,
    #[constraint(
        init,
        payer = payer,
        space = Counter::SPACE,
        seeds = [
            b"counter".as_ref(),
            args.admin.as_ref(),
        ],
        bump = args.bump,
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

#[context]
pub struct IncrementContext {
    pub payer: Signer,
    #[constraint(
        seeds = [
            b"counter".as_ref(),
            counter.data()?.admin.as_ref(),
        ]
        bump = counter.data()?.bump,
    )]
    pub counter: Mut<Account<Counter>>,
}

pub fn initialize(ctx: InitContext) -> Result<(), ProgramError> {
    *ctx.counter.mut_data()? = Counter {
        bump: ctx.args.bump,
        admin: ctx.args.admin,
        count: 0,
    };

    Ok(())
}

pub fn increment(ctx: IncrementContext) -> Result<(), ProgramError> {
    if *ctx.payer.key() != ctx.counter.data()?.admin {
        return Err(ProgramError::IllegalOwner);
    }

    ctx.counter.mut_data()?.count += 1;

    Ok(())
}

#[account]
pub struct Counter {
    pub bump: u64, // Should be u8 if 8-bit aligned
    pub admin: Pubkey,
    pub count: u64,
}

impl Counter {
    const SPACE: usize = std::mem::size_of::<Counter>();
}

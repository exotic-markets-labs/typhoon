use {
    bytemuck::{Pod, Zeroable},
    crayfish_account_macro::account,
    crayfish_accounts::{
        Account, FromAccountInfo, Mut, Program, ReadableAccount, Signer, System, WritableAccount,
    },
    crayfish_context_macro::context,
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
#[args(admin: Pubkey, bump: u64)]
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
        bump,
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
        bump: ctx.args.bump as u8,
        admin: ctx.args.admin,
        count: 0,
        _padding: [0_u8; 7],
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
    pub bump: u8,
    pub _padding: [u8; 7], // used to align fields
    pub admin: Pubkey,
    pub count: u64,
}

impl Counter {
    const SPACE: usize = std::mem::size_of::<Counter>();
}

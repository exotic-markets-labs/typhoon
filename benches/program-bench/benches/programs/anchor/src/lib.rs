use anchor_lang::prelude::*;

declare_id!("Bench111111111111111111111111111111111111111");

#[program]
pub mod lever {
    use super::*;

    #[instruction(discriminator = [0])]
    pub fn ping(_ctx: Context<Empty>) -> Result<()> {
        Ok(())
    }

    #[instruction(discriminator = [1])]
    pub fn log(_ctx: Context<Empty>) -> Result<()> {
        msg!("Instruction: Log");
        Ok(())
    }

    #[instruction(discriminator = [2])]
    pub fn create_account(ctx: Context<CreateAccount>) -> Result<()> {
        let mut acc_mut = ctx.accounts.account.load_init()?;
        acc_mut.byte = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Empty {}

#[derive(Accounts)]
pub struct CreateAccount<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        space = 8 + 1,
        payer = admin
    )]
    pub account: AccountLoader<'info, Data>,
    pub system_program: Program<'info, System>,
}

#[account(zero_copy)]
pub struct Data {
    pub byte: u8,
}

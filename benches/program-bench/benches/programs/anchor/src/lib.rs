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

    #[instruction(discriminator = [3])]
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.account.to_account_info(),
                },
            ),
            amount,
        )
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

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account(zero_copy)]
pub struct Data {
    pub byte: u8,
}

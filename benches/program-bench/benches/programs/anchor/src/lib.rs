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

    #[instruction(discriminator = [4])]
    pub fn unchecked_accounts(_ctx: Context<UncheckedAccounts>) -> Result<()> {
        Ok(())
    }

    #[instruction(discriminator = [5])]
    pub fn accounts(_ctx: Context<AccountsC>) -> Result<()> {
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

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UncheckedAccounts<'info> {
    pub account1: UncheckedAccount<'info>,
    pub account2: UncheckedAccount<'info>,
    pub account3: UncheckedAccount<'info>,
    pub account4: UncheckedAccount<'info>,
    pub account5: UncheckedAccount<'info>,
    pub account6: UncheckedAccount<'info>,
    pub account7: UncheckedAccount<'info>,
    pub account8: UncheckedAccount<'info>,
    pub account9: UncheckedAccount<'info>,
    pub account10: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct AccountsC<'info> {
    pub account1: AccountLoader<'info, Data>,
    pub account2: AccountLoader<'info, Data>,
    pub account3: AccountLoader<'info, Data>,
    pub account4: AccountLoader<'info, Data>,
    pub account5: AccountLoader<'info, Data>,
    pub account6: AccountLoader<'info, Data>,
    pub account7: AccountLoader<'info, Data>,
    pub account8: AccountLoader<'info, Data>,
    pub account9: AccountLoader<'info, Data>,
    pub account10: AccountLoader<'info, Data>,
}

#[account(zero_copy)]
pub struct Data {
    pub byte: u8,
}

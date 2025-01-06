use anchor_lang::prelude::*;

declare_id!("Bench111111111111111111111111111111111111111");

#[program]
pub mod anchor_program {
    use super::*;

    pub fn ping(_ctx: Context<PingContext>) -> Result<()> {
        Ok(())
    }

    pub fn log(_ctx: Context<LogContext>) -> Result<()> {
        msg!("Instruction: Log");
        Ok(())
    }

    pub fn account(ctx: Context<AccountContext>, expected: u64) -> Result<()> {
        if ctx.remaining_accounts.len() == expected as usize {
            Ok(())
        } else {
            Err(ProgramError::InvalidArgument.into())
        }
    }

    pub fn create_account(ctx: Context<CreateAccountContext>) -> Result<()> {
        let state = &mut ctx.accounts.to.load_init()?;

        state.data = 35;

        Ok(())
    }

    pub fn transfer(ctx: Context<TransferContext>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PingContext {}

#[derive(Accounts)]
pub struct LogContext {}

#[derive(Accounts)]
pub struct AccountContext {}

#[derive(Accounts)]
pub struct CreateAccountContext<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(init, payer = from, space = 8 + 8)]
    pub to: AccountLoader<'info, AccountState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferContext {}

#[account(zero_copy)]
pub struct AccountState {
    pub data: u64,
}
/*
    // 3 - CreateAccount
    Some((&3, [])) => Ok(Instruction::CreateAccount),
    // 4 - Transfer
    Some((&4, [])) => Ok(Instruction::Transfer),
    _ => Err(ProgramError::InvalidInstructionData),
*/

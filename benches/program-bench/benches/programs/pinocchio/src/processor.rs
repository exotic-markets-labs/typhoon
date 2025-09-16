use {
    core::ptr,
    pinocchio::{
        account_info::AccountInfo,
        msg,
        program_error::ProgramError,
        sysvars::{rent::Rent, Sysvar},
        ProgramResult,
    },
    pinocchio_system::instructions::{Allocate, Assign, CreateAccount, Transfer},
};

const ACCOUNT_DISCRIMINATOR: [u8; 8] = [206, 156, 59, 188, 18, 79, 240, 232];

#[inline(always)]
pub fn process_ping() -> ProgramResult {
    Ok(())
}

#[inline(always)]
pub fn process_log() -> ProgramResult {
    msg!("Instruction: Log");
    Ok(())
}

#[inline(always)]
pub fn process_create_account(accounts: &[AccountInfo]) -> ProgramResult {
    let [payer, to, _rem @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let rent = Rent::get()?;
    let current_lamports = to.lamports();
    if current_lamports == 0 {
        CreateAccount {
            from: payer,
            lamports: rent.minimum_balance(9),
            owner: &crate::ID,
            space: 9,
            to,
        }
        .invoke();
    } else {
        let required_lamports = rent
            .minimum_balance(9)
            .max(1)
            .saturating_sub(current_lamports);

        if required_lamports > 0 {
            Transfer {
                from: payer,
                to,
                lamports: required_lamports,
            }
            .invoke()?;
        }

        Allocate {
            account: to,
            space: 9,
        }
        .invoke()?;

        Assign {
            account: to,
            owner: &crate::ID,
        }
        .invoke()?;
    }
    let mut data = to.try_borrow_mut_data()?;
    data[0..8].copy_from_slice(&ACCOUNT_DISCRIMINATOR);
    data[8] = 1;

    Ok(())
}

#[inline(always)]
pub fn process_transfer(instruction_data: &[u8], accounts: &[AccountInfo]) -> ProgramResult {
    Transfer {
        from: &accounts[0],
        to: &accounts[1],
        lamports: u64::from_le_bytes(
            instruction_data[0..8]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        ),
    }
    .invoke()
}

#[inline(always)]
pub fn process_unchecked_accounts(_accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

#[inline(always)]
pub fn process_accounts(accounts: &[AccountInfo]) -> ProgramResult {
    for account in accounts {
        if account.owner() != &crate::ID {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if account.data_len() < 9 {
            return Err(ProgramError::InvalidAccountData);
        }
        if unsafe { ptr::read_unaligned::<u64>(account.data_ptr() as *const u64) }
            != u64::from_le_bytes(ACCOUNT_DISCRIMINATOR)
        {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    Ok(())
}

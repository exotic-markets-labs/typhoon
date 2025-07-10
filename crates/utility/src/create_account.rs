use {
    pinocchio::{
        account_info::AccountInfo, instruction::Signer, pubkey::Pubkey, sysvars::rent::Rent,
    },
    pinocchio_system::instructions::{Allocate, Assign, CreateAccount, Transfer},
    typhoon_accounts::WritableAccount,
    typhoon_errors::{Error, ErrorCode},
};

#[inline(always)]
pub fn create_or_assign(
    account: &AccountInfo,
    rent: &Rent,
    payer: &impl WritableAccount,
    owner: &Pubkey,
    space: usize,
    seeds: Option<&[Signer]>,
) -> Result<(), Error> {
    let current_lamports = account.lamports();
    if current_lamports == 0 {
        CreateAccount {
            from: payer.as_ref(),
            lamports: rent.minimum_balance(space),
            owner,
            space: space as u64,
            to: account,
        }
        .invoke_signed(seeds.unwrap_or_default())?;
    } else {
        if payer.key() == account.key() {
            return Err(ErrorCode::TryingToInitPayerAsProgramAccount.into());
        }

        let required_lamports = rent
            .minimum_balance(space)
            .max(1)
            .saturating_sub(current_lamports);

        if required_lamports > 0 {
            Transfer {
                from: payer.as_ref(),
                to: account,
                lamports: required_lamports,
            }
            .invoke()?;
        }

        Allocate {
            account,
            space: space as u64,
        }
        .invoke_signed(seeds.unwrap_or_default())?;

        Assign { account, owner }.invoke_signed(seeds.unwrap_or_default())?;
    }

    Ok(())
}

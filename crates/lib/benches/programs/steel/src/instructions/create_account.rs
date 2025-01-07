use steel::sysvar::rent::Rent;
use steel::*;

pub fn c_account(accounts: &[AccountInfo]) -> ProgramResult {
    let [payer, new_account, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer
        .is_signer()
        .map_err(|_| ProgramError::MissingRequiredSignature)?;

    new_account.is_signer()?.is_empty()?.is_writable()?;
    system_program.is_program(&system_program::ID)?;

    let space_required = 10;
    let lamports_required = Rent::get()?.minimum_balance(space_required as usize);

    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            payer.key,
            new_account.key,
            lamports_required,
            space_required,
            &system_program::ID,
        ),
        &[payer.clone(), new_account.clone(), system_program.clone()],
    )?;

    Ok(())
}

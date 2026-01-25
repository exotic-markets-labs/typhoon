use typhoon::prelude::*;

pub fn account_iter(accounts: AccountIter<(SystemAccount,)>) -> ProgramResult {
    for (acc,) in accounts {
        acc.address().log()
    }
    Ok(())
}

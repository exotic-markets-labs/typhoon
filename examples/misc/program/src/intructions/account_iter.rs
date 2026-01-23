use solana_program_log::log;
use typhoon::prelude::*;

pub fn account_iter(accounts: AccountIter<(SystemAccount,)>) -> ProgramResult {
    for (acc,) in accounts {
        log(acc.address());
    }
    Ok(())
}

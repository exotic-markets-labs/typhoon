use star_frame::prelude::*;

#[derive(StarFrameProgram)]
#[program(
    instruction_set = StarFrameInstructionSet,
    id = "Bench111111111111111111111111111111111111111"
)]
pub struct CounterProgram;

#[derive(InstructionSet)]
#[ix_set(use_repr)]
#[repr(u8)]
pub enum StarFrameInstructionSet {
    Ping(()),
    Log(Log),
    CreateAccount(CreateAccount),
    Transfer(Transfer),
    UncheckedAccounts(UncheckedAccounts),
    Accounts(Accounts),
}
#[derive(Debug, InstructionArgs, BorshDeserialize)]
pub struct Log;

#[star_frame_instruction]
fn Log(_accounts: &mut ()) -> Result<()> {
    msg!("Instruction: Log");
    Ok(())
}

#[zero_copy(pod, skip_packed)]
#[derive(ProgramAccount, Debug)]
pub struct Data {
    pub byte: u8,
}

#[derive(InstructionArgs, BorshDeserialize)]
pub struct CreateAccount;

#[derive(AccountSet)]
pub struct CreateAccountAccounts {
    pub authority: Mut<Signer>,
    #[validate(arg = Create((&self.authority,)))]
    pub counter: Init<Signer<Account<Data>>>,
    pub system_program: Program<System>,
}

#[star_frame_instruction]
fn CreateAccount(accounts: &mut CreateAccountAccounts) -> Result<()> {
    accounts.counter.data_mut()?.byte = 1;
    Ok(())
}

#[derive(AccountSet)]
pub struct TransferAccounts {
    pub admin: Mut<Signer>,
    pub account: Mut<SystemAccount>,
    pub system_program: Program<System>,
}

#[derive(InstructionArgs, BorshDeserialize)]
pub struct Transfer {
    #[ix_args(run)]
    amount: u64,
}

#[star_frame_instruction]
fn Transfer(accounts: &mut TransferAccounts, lamports: u64) -> Result<()> {
    System::cpi(
        star_frame::program::system::Transfer { lamports },
        star_frame::program::system::TransferCpiAccounts {
            funder: *accounts.admin.account_info(),
            recipient: *accounts.account.account_info(),
        },
        None,
    )
    .invoke()?;
    Ok(())
}

#[derive(AccountSet)]
pub struct UncheckedAccountsAccounts {
    pub account1: AccountInfo,
    pub account2: AccountInfo,
    pub account3: AccountInfo,
    pub account4: AccountInfo,
    pub account5: AccountInfo,
    pub account6: AccountInfo,
    pub account7: AccountInfo,
    pub account8: AccountInfo,
    pub account9: AccountInfo,
    pub account10: AccountInfo,
}

#[derive(InstructionArgs, BorshDeserialize)]
pub struct UncheckedAccounts;

#[star_frame_instruction]
fn UncheckedAccounts(_accounts: &mut UncheckedAccountsAccounts) -> Result<()> {
    Ok(())
}

#[derive(AccountSet)]
pub struct AccountsAccounts {
    pub account1: Account<Data>,
    pub account2: Account<Data>,
    pub account3: Account<Data>,
    pub account4: Account<Data>,
    pub account5: Account<Data>,
    pub account6: Account<Data>,
    pub account7: Account<Data>,
    pub account8: Account<Data>,
    pub account9: Account<Data>,
    pub account10: Account<Data>,
}

#[derive(InstructionArgs, BorshDeserialize)]
pub struct Accounts;

#[star_frame_instruction]
fn Accounts(_accounts: &mut AccountsAccounts) -> Result<()> {
    Ok(())
}

use star_frame::prelude::*;

#[derive(StarFrameProgram)]
#[program(
    instruction_set = StarFrameInstructionSet,
    id = "Bench111111111111111111111111111111111111111"
)]
pub struct CounterProgram;

#[derive(InstructionSet)]
pub enum StarFrameInstructionSet {
    Ping(Ping),
    Log(Log),
    CreateAccount(CreateAccount),
    Transfer(Transfer),
    UncheckedAccounts(UncheckedAccounts),
    Accounts(Accounts),
}

#[derive(AccountSet)]
pub struct Empty;

#[derive(InstructionArgs, BorshDeserialize)]
pub struct Ping;

impl StarFrameInstruction for Ping {
    type ReturnType = ();
    type Accounts<'b, 'c> = Empty;

    fn process(
        _accounts: &mut Self::Accounts<'_, '_>,
        _run_arg: Self::RunArg<'_>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {
        Ok(())
    }
}

#[derive(InstructionArgs, BorshDeserialize)]
pub struct Log;

impl StarFrameInstruction for Log {
    type ReturnType = ();
    type Accounts<'b, 'c> = ();

    fn process(
        _accounts: &mut Self::Accounts<'_, '_>,
        _run_arg: Self::RunArg<'_>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {
        msg!("Instruction: Log");
        Ok(())
    }
}

#[derive(Copy, Clone, ProgramAccount, CheckedBitPattern, NoUninit, Align1, Zeroable, Debug)]
#[repr(C)]
pub struct Data {
    pub byte: u8,
}

#[derive(InstructionArgs, BorshDeserialize)]
pub struct CreateAccount;

#[derive(AccountSet)]
pub struct CreateAccountAccounts {
    #[validate(funder)]
    pub authority: Signer<Mut<SystemAccount>>,
    #[validate(arg = Create(()))]
    pub counter: Init<Signer<Account<Data>>>,
    pub system_program: Program<System>,
}

impl StarFrameInstruction for CreateAccount {
    type ReturnType = ();
    type Accounts<'b, 'c> = CreateAccountAccounts;

    fn process(
        accounts: &mut Self::Accounts<'_, '_>,
        _args: Self::RunArg<'_>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {
        accounts.counter.data_mut()?.byte = 1;
        Ok(())
    }
}

#[derive(AccountSet)]
pub struct TransferAccounts {
    #[validate(funder)]
    pub admin: Signer<Mut<SystemAccount>>,
    pub account: Mut<SystemAccount>,
    pub system_program: Program<System>,
}

#[derive(InstructionArgs, BorshDeserialize)]
pub struct Transfer {
    #[ix_args(&run)]
    amount: u64,
}

impl StarFrameInstruction for Transfer {
    type ReturnType = ();
    type Accounts<'b, 'c> = TransferAccounts;

    fn process(
        accounts: &mut Self::Accounts<'_, '_>,
        amount: &u64,
        ctx: &mut Context,
    ) -> Result<Self::ReturnType> {
        System::cpi(
            &star_frame::program::system::Transfer { lamports: *amount },
            star_frame::program::system::TransferCpiAccounts {
                funder: ***accounts.admin,
                recipient: **accounts.account,
            },
            ctx,
        )?
        .invoke()?;
        Ok(())
    }
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

impl StarFrameInstruction for UncheckedAccounts {
    type ReturnType = ();
    type Accounts<'b, 'c> = UncheckedAccountsAccounts;

    fn process(
        _accounts: &mut Self::Accounts<'_, '_>,
        _amount: Self::RunArg<'_>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {
        Ok(())
    }
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

impl StarFrameInstruction for Accounts {
    type ReturnType = ();
    type Accounts<'b, 'c> = AccountsAccounts;

    fn process(
        _accounts: &mut Self::Accounts<'_, '_>,
        _amount: Self::RunArg<'_>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {
        Ok(())
    }
}

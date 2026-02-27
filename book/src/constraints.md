# Constraints

Constraints are a powerful feature in Typhoon that allow you to declaratively define validation and initialization logic for your instruction accounts. They are applied using the `#[constraint(...)]` attribute on fields within a `#[context]` struct.

At compile time, the macro expands constraints into efficient validation code, eliminating boilerplate while keeping your programs safe.

## Quick Reference

| Constraint | Syntax | Description |
|---|---|---|
| [`init`](#init) | `init` | Initialize a new account |
| [`init_if_needed`](#init_if_needed) | `init_if_needed` | Initialize only if account doesn't exist |
| [`payer`](#payer) | `payer = <field>` | Account that pays for initialization |
| [`space`](#space) | `space = <expr>` | Allocated byte size for new accounts |
| [`seeds`](#seeds) | `seeds = [...]` | PDA seed derivation |
| [`seeded`](#seeded) | `seeded` / `seeded = [...]` | PDA derivation via the `Seeded` trait |
| [`bump`](#bump) | `bump` / `bump = <expr>` | PDA bump seed |
| [`program`](#program) | `program = <expr>` | Program ID for PDA derivation |
| [`has_one`](#has_one) | `has_one = <field>` | Validate account data field matches another account |
| [`assert`](#assert) | `assert = <expr>` | Custom assertion on account data |
| [`address`](#address) | `address = <expr>` | Validate account address |
| [`token::*`](#token-constraints) | `token::mint = ...` / `token::owner = ...` | Token account validation |
| [`mint::*`](#mint-constraints) | `mint::decimals = ...` / `mint::authority = ...` / `mint::freeze_authority = ...` | Mint account configuration |
| [`associated_token::*`](#associated-token-constraints) | `associated_token::mint = ...` / `associated_token::authority = ...` | Associated token account derivation |

---

## Account Initialization

### `init`

Marks an account to be created and initialized. The account must be wrapped in `Mut<>` and must be a signer (either a keypair signer via `SignerNoCheck<>` or a PDA via `seeds`).

Requires [`payer`](#payer). Optionally takes [`space`](#space) (defaults to `AccountType::SPACE`).

The system program is automatically required when `init` is used.

```rust
#[context]
pub struct InitCounter {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
    )]
    pub counter: Mut<SignerNoCheck<Account<Counter>>>,
    pub system: Program<System>,
}
```

When initializing a PDA, combine `init` with [`seeds`](#seeds) and [`bump`](#bump):

```rust
#[context]
pub struct InitPda {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
        space = Counter::SPACE,
        seeds = [b"counter".as_ref()],
        bump
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}
```

### `init_if_needed`

Conditionally initializes an account — only if it does not already exist. If the account is already initialized, it validates the account as normal instead.

This is particularly useful for associated token accounts that may or may not exist when the instruction is called.

```rust
#[context]
pub struct MaybeCreate {
    pub payer: Mut<Signer>,
    #[constraint(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub token_account: Mut<Account<TokenAccount>>,
    pub mint: Account<Mint>,
    pub owner: UncheckedAccount,
    pub ata_program: Program<AtaTokenProgram>,
    pub token_program: Program<TokenProgram>,
    pub system_program: Program<System>,
}
```

### `payer`

Specifies which account pays the rent for a newly created account. The payer must be a `Mut<Signer>`.

**Syntax**: `payer = <field_name>`

```rust
#[constraint(
    init,
    payer = authority,   // `authority` field pays for this account
)]
```

### `space`

Sets the number of bytes to allocate for the new account. If omitted, defaults to `AccountType::SPACE` which is derived from the struct size plus the 8-byte discriminator.

**Syntax**: `space = <expr>`

```rust
#[constraint(
    init,
    payer = payer,
    space = 8 + core::mem::size_of::<MyData>()
)]
pub data: Mut<SignerNoCheck<Account<MyData>>>,
```

You can also reference a constant:

```rust
impl MyData {
    const SPACE: usize = 8 + core::mem::size_of::<MyData>();
}

// ...
#[constraint(
    init,
    payer = payer,
    space = MyData::SPACE
)]
```

---

## PDA Constraints

### `seeds`

Defines the seeds used to derive a Program Derived Address (PDA). Seeds can be specified as an array of byte slices or as a function call that returns the seeds.

**Syntax**: `seeds = [<expr>, ...]` or `seeds = <fn_call>()`

**Array form** — each element must be a byte slice (`&[u8]`):

```rust
#[constraint(
    seeds = [
        b"escrow",
        maker.address().as_ref(),
        &args.seed.to_le_bytes(),
    ],
    bump
)]
pub escrow: Mut<Account<Escrow>>,
```

**Function form** — a function returning a seed array:

```rust
fn pda_seeds<'a>() -> [&'a [u8]; 1] {
    [b"counter".as_ref()]
}

// ...
#[constraint(
    init,
    payer = payer,
    seeds = pda_seeds(),
    bump
)]
pub counter: Mut<Account<Counter>>,
```

### `seeded`

Uses the `Seeded` trait (derived from `#[key]` attributes on your account struct) to automatically derive PDA seeds from the account's state fields.

**Syntax**: `seeded` or `seeded = [<additional_seeds>]`

First, define your account struct with `#[key]` fields:

```rust
#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
#[no_space]
pub struct Counter {
    #[key]
    pub admin: Address,
    pub bump: u8,
    _padding: [u8; 7],
    pub count: u64,
}
```

Then use `seeded` in your constraints. Without arguments, it derives seeds from the existing account data:

```rust
#[context]
pub struct Increment {
    pub admin: Signer,
    #[constraint(seeded, bump = counter.data()?.bump, has_one = admin)]
    pub counter: Mut<Account<Counter>>,
}
```

With additional seeds for initialization (where the account data doesn't exist yet):

```rust
#[context]
#[args(admin: Address, bump: u8)]
pub struct Init {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
        space = Counter::SPACE,
        seeded = [&args.admin],
        bump
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}
```

### `bump`

Controls how the PDA bump is resolved. Used with [`seeds`](#seeds) or [`seeded`](#seeded).

**Syntax**: `bump` or `bump = <expr>`

**Without a value** — calls `find_program_address` to discover both the PDA address and bump. The bump is stored in `ctx.bumps.<field_name>` for later use:

```rust
#[constraint(
    init,
    payer = payer,
    seeds = [b"counter".as_ref()],
    bump     // finds the bump automatically
)]
pub counter: Mut<Account<Counter>>,

// In the handler:
pub fn initialize(ctx: Init) -> ProgramResult {
    // Access the discovered bump
    ctx.counter.mut_data()?.bump = ctx.bumps.counter;
    Ok(())
}
```

**With a value** — uses `create_program_address` with the provided bump, which is cheaper since it doesn't need to iterate:

```rust
#[constraint(
    seeds = [b"counter".as_ref()],
    bump = counter.data()?.bump   // use stored bump
)]
pub counter: Mut<Account<Counter>>,
```

### `program`

Overrides the program ID used for PDA derivation. By default, the current program's ID is used. This is useful when verifying PDAs owned by other programs.

**Syntax**: `program = <expr>`

```rust
#[constraint(
    seeds = [b"metadata", authority.address().as_ref()],
    bump,
    program = other_program_id
)]
pub metadata: Account<Metadata>,
```

---

## Validation Constraints

### `has_one`

Validates that a field stored in the account's deserialized data matches the address of another account in the context. The account data field must have the same name as the target context field.

**Syntax**: `has_one = <field>` or `has_one = <field> @ <error_expr>`

For this to work, the account struct must have a field with the same name as the referenced context field. At runtime, Typhoon checks that the stored address equals the context account's address.

```rust
#[derive(AccountState, NoUninit, AnyBitPattern, Clone, Copy)]
#[repr(C)]
pub struct Counter {
    pub count: u64,
    pub admin: Address,   // Must match `admin` account in context
    pub bump: u8,
    _padding: [u8; 7],
}

#[context]
pub struct Increment {
    pub admin: Signer,    // Checked against counter.admin
    #[constraint(
        has_one = admin,
        seeds = [b"counter".as_ref()],
        bump = counter.data()?.bump,
    )]
    pub counter: Mut<Account<Counter>>,
}
```

**With a custom error**:

```rust
#[derive(TyphoonError)]
pub enum MyError {
    #[msg("Error: Invalid owner")]
    InvalidOwner = 200,
}

#[constraint(
    has_one = admin @ MyError::InvalidOwner,
)]
```

You can also use built-in Solana errors:

```rust
#[constraint(
    has_one = admin @ ProgramError::IllegalOwner,
)]
```

### `assert`

Evaluates a custom boolean expression against account data. The assertion fails the transaction if the expression evaluates to `false`.

**Syntax**: `assert = <expr>` or `assert = <expr> @ <error_expr>`

You can access deserialized account data via the `.data()?` method:

```rust
#[context]
pub struct Simple {
    #[constraint(
        assert = account.data()?.counter == 1,
    )]
    pub account: Account<RandomData>,
}
```

**With a custom error**:

```rust
#[constraint(
    assert = account.data()?.counter > 0 @ MyError::InvalidCounter,
)]
```

### `address`

Validates that the account's public key matches the given address expression.

**Syntax**: `address = <expr>` or `address = <expr> @ <error_expr>`

Useful for checking against known constant addresses (e.g., compile-time derived PDAs):

```rust
use const_crypto::ed25519;

pub const RANDOM_PDA: (Address, u8) = {
    let (key, bump) = ed25519::derive_program_address(&[b"random"], crate::ID.as_array());
    (Address::new_from_array(key), bump)
};

#[context]
pub struct Verify {
    #[constraint(
        address = &RANDOM_PDA.0
    )]
    pub account: Account<RandomData>,
}
```

Using `address` with [`assert`](#assert) together:

```rust
#[context]
pub struct Simple {
    #[constraint(
        assert = account.data()?.counter == 1,
        address = &RANDOM_PDA.0
    )]
    pub account: Account<RandomData>,
}
```

---

## SPL Token Constraints

These constraints are used when working with the SPL Token program. They require the `typhoon-token` crate.

### Token Constraints

Validate and configure SPL token accounts. Use the `token::` prefix.

#### `token::mint`

Validates that the token account's mint matches the given mint account.

**Syntax**: `token::mint = <field>`

#### `token::owner`

Validates that the token account's owner matches the given expression.

**Syntax**: `token::owner = <expr>`

**Example** — validating a token account:

```rust
use typhoon_token::{TokenAccount, TokenProgram};

#[context]
pub struct ValidateToken {
    pub authority: Signer,
    pub mint: Account<Mint>,
    #[constraint(
        token::mint = mint,
        token::owner = authority.address(),
    )]
    pub token_account: Account<TokenAccount>,
}
```

### Mint Constraints

Configure SPL mint accounts during initialization. Use the `mint::` prefix.

#### `mint::decimals`

Sets the number of decimals for the mint. Defaults to `9` if not specified during initialization.

**Syntax**: `mint::decimals = <expr>`

#### `mint::authority`

Sets the mint authority.

**Syntax**: `mint::authority = <expr>`

#### `mint::freeze_authority`

Sets the freeze authority for the mint. Optional — if omitted, no freeze authority is set.

**Syntax**: `mint::freeze_authority = <expr>`

**Example** — initializing a mint:

```rust
use typhoon_token::{Mint, TokenProgram, SplCreateMint};

#[context]
#[args(MintArgs)]
pub struct CreateMint {
    pub payer: Mut<Signer>,
    pub owner: UncheckedAccount,
    #[constraint(
        init,
        payer = payer,
        mint::decimals = args.decimals,
        mint::authority = escrow.address(),
        mint::freeze_authority = owner.address()
    )]
    pub mint: Mut<SignerNoCheck<Account<Mint>>>,
    pub escrow: Account<Escrow>,
    pub token_program: Interface<TokenProgram>,
    pub system_program: Program<System>,
}
```

### Associated Token Constraints

Derive and validate associated token account (ATA) addresses. Use the `associated_token::` prefix.

#### `associated_token::mint`

Specifies which mint the ATA is associated with.

**Syntax**: `associated_token::mint = <field>`

#### `associated_token::authority`

Specifies the wallet/authority that owns the ATA.

**Syntax**: `associated_token::authority = <field>`

When combined with `init` or `init_if_needed`, the ATA is created automatically.

**Example** — creating an ATA if it doesn't exist:

```rust
use typhoon_token::{
    AtaTokenProgram, Mint, SplCreateToken, TokenAccount, TokenProgram,
};

#[context]
pub struct CreateVault {
    pub payer: Mut<Signer>,
    pub mint: Account<Mint>,
    pub owner: Signer,
    #[constraint(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub vault: Mut<Account<TokenAccount>>,
    pub ata_program: Program<AtaTokenProgram>,
    pub token_program: Program<TokenProgram>,
    pub system_program: Program<System>,
}
```

**Example** — validating an existing ATA (without initialization):

```rust
#[context]
pub struct Refund {
    pub maker: Mut<Signer>,
    pub escrow: Mut<Account<Escrow>>,
    pub mint_a: UncheckedAccount,
    #[constraint(
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: Mut<Account<TokenAccount>>,
    pub token_program: Program<TokenProgram>,
}
```

---

## Custom Errors

Several constraints support attaching a custom error using the `@` syntax. When the constraint check fails, the provided error is returned instead of the default.

**Supported constraints**: `has_one`, `assert`, `address`

**Syntax**: `<constraint> @ <error_expr>`

```rust
// Using a custom TyphoonError
#[derive(TyphoonError)]
pub enum MyError {
    #[msg("The admin does not match")]
    InvalidAdmin = 200,
    #[msg("Counter must be positive")]
    InvalidCounter = 201,
}

#[context]
pub struct Guarded {
    pub admin: Signer,
    #[constraint(
        has_one = admin @ MyError::InvalidAdmin,
        assert = counter.data()?.count > 0 @ MyError::InvalidCounter,
    )]
    pub counter: Mut<Account<Counter>>,
}
```

---

## The `bumps` Field

When you use `bump` without providing a value (triggering `find_program_address`), Typhoon automatically generates a `bumps` struct on the context. Each PDA field gets a corresponding `u8` bump value.

```rust
#[context]
pub struct Init {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
        seeds = [b"escrow", maker.address().as_ref(), &args.seed.to_le_bytes()],
        bump       // <-- bump is discovered and stored in ctx.bumps.escrow
    )]
    pub escrow: Mut<Account<Escrow>>,
    pub system: Program<System>,
}

pub fn initialize(ctx: Init) -> ProgramResult {
    *ctx.escrow.mut_data()? = Escrow {
        bump: ctx.bumps.escrow,   // save the bump for future use
        // ...
    };
    Ok(())
}
```

---

## Full Example: Escrow Program

Here's a realistic example combining multiple constraints in an escrow program:

```rust
use {
    escrow_interface::{state::Escrow, MakeArgs},
    typhoon::prelude::*,
    typhoon_token::{spl_instructions::Transfer, *},
};

#[context]
#[args(MakeArgs)]
pub struct Make {
    pub maker: Mut<Signer>,
    #[constraint(
        init,
        payer = maker,
        seeds = [b"escrow", maker.address().as_ref(), &args.seed.to_le_bytes()],
        bump
    )]
    pub escrow: Mut<Account<Escrow>>,
    pub mint_a: Account<Mint>,
    pub mint_b: Account<Mint>,
    pub maker_ata_a: Mut<Account<TokenAccount>>,
    #[constraint(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: Mut<Account<TokenAccount>>,
    pub ata_program: Program<AtaTokenProgram>,
    pub token_program: Program<TokenProgram>,
    pub system_program: Program<System>,
}

pub fn make(ctx: Make) -> ProgramResult {
    *ctx.escrow.mut_data()? = Escrow {
        maker: *ctx.maker.address(),
        mint_a: *ctx.mint_a.address(),
        mint_b: *ctx.mint_b.address(),
        seed: ctx.args.seed,
        receive: ctx.args.receive,
        bump: ctx.bumps.escrow,
    };

    Transfer {
        from: ctx.maker_ata_a.as_ref(),
        to: ctx.vault.as_ref(),
        authority: ctx.maker.as_ref(),
        amount: ctx.args.amount,
    }
    .invoke()?;

    Ok(())
}
```

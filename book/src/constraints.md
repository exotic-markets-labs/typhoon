# Constraints

Constraints are a powerful feature in Typhoon that allow you to declaratively define validation logic for your instruction accounts. They are used within the `#[context]` macro.

## Basic Usage

Constraints are added using the `#[constraint(...)]` attribute on fields in your context struct.

```rust
#[context]
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
    )]
    pub counter: Mut<SignerNoCheck<Account<Counter>>>,
    pub system: Program<System>,
}
```

## Common Constraints

- **`init`**: Marks an account to be initialized. Requires a `payer`.
- **`payer = <field>`**: Specifies which account pays for the creation of an initialized account.
- **`seeds = [...]`**: Defines the seeds for a Program Derived Address (PDA).
- **`bump = <value>`**: Sets or validates the bump for a PDA.
- **`owner = <address>`**: Ensures the account is owned by a specific program.

Constraints significantly reduce boilerplate and make your programs more secure by centralizing validation logic.

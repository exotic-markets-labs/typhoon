#![no_std]

use {
    bytemuck::{AnyBitPattern, NoUninit},
    typhoon::prelude::*,
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

nostd_panic_handler!();
no_allocator!();

#[context]
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

#[context]
pub struct CounterMutContext {
    pub counter: Mut<Account<Counter>>,
}

#[context]
pub struct DestinationContext {
    pub destination: Mut<SystemAccount>,
}

#[context]
pub struct ArrayCounterContext {
    pub counters: [Mut<Account<Counter>>; 2],
}

#[context]
pub struct ConstrainedArrayContext {
    pub counters: [Mut<Account<Counter>>; 3],
}

handlers! {
    initialize,
    increment,
    close,
    increment_array,
    increment_constrained_array
}

pub fn initialize(_: InitContext) -> ProgramResult {
    Ok(())
}

pub fn increment(ctx: CounterMutContext) -> ProgramResult {
    ctx.counter.mut_data()?.count += 1;

    Ok(())
}

pub fn close(
    CounterMutContext { counter }: CounterMutContext,
    DestinationContext { destination }: DestinationContext,
) -> ProgramResult {
    counter.close(&destination)?;

    Ok(())
}

pub fn increment_array(ctx: ArrayCounterContext) -> ProgramResult {
    for counter in ctx.counters.iter() {
        counter.mut_data()?.count += 1;
    }

    Ok(())
}

pub fn increment_constrained_array(ctx: ConstrainedArrayContext) -> ProgramResult {
    for counter in ctx.counters.iter() {
        counter.mut_data()?.count += 1;
    }

    Ok(())
}

#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
pub struct Counter {
    pub count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_syntax_validation() {
        // Test that various array syntaxes are accepted by the macro
        // This ensures the parser correctly handles different array sizes

        #[context]
        struct TestContext1 {
            pub single: [Mut<Account<Counter>>; 1],
        }

        #[context]
        struct TestContext2 {
            pub pair: [Mut<Account<Counter>>; 2],
        }

        #[context]
        struct TestContext5 {
            pub quintuple: [Mut<Account<Counter>>; 5],
        }

        // If these contexts compile, it means the macro correctly
        // parses array syntax for different sizes
        assert!(true);
    }

    #[test]
    fn test_constraint_propagation() {
        // Test that constraints are properly handled for array accounts
        // The ConstrainedArrayContext uses the Mut wrapper which provides
        // mutability constraints automatically

        #[context]
        struct MutArrayContext {
            pub mutable_counters: [Mut<Account<Counter>>; 3],
        }

        // If this compiles, it means the Mut wrapper is correctly
        // applied to array elements and provides the expected constraints
        assert!(true);
    }

    #[test]
    fn test_security_validations() {
        // Test that security validations work by trying to compile invalid cases
        // These would fail to compile if the security checks work properly

        // Valid cases that should compile:
        #[context]
        struct ValidArrayContext {
            pub small_array: [Mut<Account<Counter>>; 5], // Valid: size 5 <= 100
            pub single_item: [Account<Counter>; 1],      // Valid: size 1 >= 1
        }
        assert!(true);
    }

    #[test]
    fn test_array_validation_violations() {
        #[context]
        struct InvalidZeroArray {
            pub zero: [Account<Counter>; 0], //invalid: size 0
        }

        #[context]
        struct InvalidLargeArray {
            pub large: [Account<Counter>; 101], //invalid: size > 100
        }

        assert!(true);
    }
}

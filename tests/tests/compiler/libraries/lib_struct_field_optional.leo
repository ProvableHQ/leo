// Test: a library top-level struct whose member type is `Optional<primitive>`.
//
// Exercises phase-1 `collect_composite` for a library struct that itself needs
// an Optional wrapper synthesized for a primitive member type. The wrapper must
// be owned by the library (the unit that first requests it) and drained onto
// the library's top-level `structs` list.

// --- library: token_lib --- //

struct Balance {
    amount: u64,
    supply: u64?,
}

fn make_empty(amount: u64) -> Balance {
    return Balance { amount, supply: none };
}

fn amount_of(b: Balance) -> u64 {
    return b.amount;
}

// --- Next Program --- //

program bank.aleo {
    fn compute(amount: u64) -> u64 {
        let b = token_lib::make_empty(amount);
        return token_lib::amount_of(b);
    }

    @noupgrade
    constructor() {}
}

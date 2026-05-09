/*
[case]
program = "test.aleo"
function = "sum_first_three"
input = []
seed_mapping = ["entries__len__", "false", "3u32"]
seed_mapping = ["entries__", "0u32", "10u64"]
seed_mapping = ["entries__", "1u32", "20u64"]
seed_mapping = ["entries__", "2u32", "30u64"]

[case]
program = "test.aleo"
function = "weighted_balance"
input = ["aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "5u32"]
seed_mapping = ["balances", "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "100u64"]

[case]
program = "test.aleo"
function = "capped_balance"
input = ["aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "50u64"]
seed_mapping = ["balances", "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "100u64"]

[case]
program = "test.aleo"
function = "capped_balance"
input = ["aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "200u64"]
seed_mapping = ["balances", "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "100u64"]
*/

// Queries that do something more interesting than a single mapping read:
// loops over a vector, branching on input bounds, multi-input arithmetic.
program test.aleo {
    mapping balances: address => u64;
    storage entries: [u64];

    fn dummy(x: u32) -> u32 { return x; }

    // Sum of the first three vector entries (or 0 for missing slots).
    // Demonstrates a Leo `for` loop unrolling inside a query.
    query fn sum_first_three() -> u64 {
        let total: u64 = 0u64;
        for i: u32 in 0u32..3u32 {
            total += entries.get(i).unwrap_or(0u64);
        }
        return total;
    }

    // Balance times an integer multiplier passed in as an input.
    // Demonstrates two inputs and arithmetic on a mapping read.
    query fn weighted_balance(addr: address, weight: u32) -> u64 {
        let b: u64 = balances.get_or_use(addr, 0u64);
        return b * weight as u64;
    }

    // Returns min(balance, cap). Demonstrates a real branching decision
    // (lowered to a ternary by the flattening pass).
    query fn capped_balance(addr: address, cap: u64) -> u64 {
        let raw: u64 = balances.get_or_use(addr, 0u64);
        if raw > cap {
            return cap;
        } else {
            return raw;
        }
    }

    @noupgrade
    constructor() {}
}

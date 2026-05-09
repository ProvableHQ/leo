/*
[case]
program = "test.aleo"
function = "current_owner"
input = []

[case]
program = "test.aleo"
function = "current_owner"
input = []
seed_mapping = ["owner__", "false", "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9"]

[case]
program = "test.aleo"
function = "entry_count"
input = []
seed_mapping = ["entries__len__", "false", "3u32"]

[case]
program = "test.aleo"
function = "entry_at"
input = ["1u32"]
seed_mapping = ["entries__len__", "false", "3u32"]
seed_mapping = ["entries__", "0u32", "10u64"]
seed_mapping = ["entries__", "1u32", "20u64"]
seed_mapping = ["entries__", "2u32", "30u64"]

[case]
program = "test.aleo"
function = "entry_at_oob"
input = ["5u32"]
seed_mapping = ["entries__len__", "false", "3u32"]
*/

// Queries that read storage variables and vectors. Storage and vector reads
// lower to mapping reads with internal naming conventions
// (`<name>__` for the data, `<name>__len__` for vector lengths), so the
// `seed_mapping` entries below address the lowered mapping names directly.
program test.aleo {
    storage owner: address;
    storage entries: [u64];

    fn dummy(x: u32) -> u32 { return x; }

    // Returns the stored owner, or a sentinel zero-address if unset.
    query fn current_owner() -> address {
        return owner.unwrap_or(aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc);
    }

    query fn entry_count() -> u32 {
        return entries.len();
    }

    // Bounds-checked read: returns the entry at `idx`, or 0 if out-of-bounds.
    query fn entry_at(idx: u32) -> u64 {
        return entries.get(idx).unwrap_or(0u64);
    }

    // Same as entry_at but the index is intentionally past the end.
    // Demonstrates the optional-default fallback path.
    query fn entry_at_oob(idx: u32) -> u64 {
        return entries.get(idx).unwrap_or(99u64);
    }

    @noupgrade
    constructor() {}
}

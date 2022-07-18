/*
namespace: Compile
expectation: Fail
*/

// This record does not define the `owner` variable required for a record type.
record Token {
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    amount: u64,
}

function main() -> bool {
    return true;
}
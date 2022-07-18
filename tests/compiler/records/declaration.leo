/*
namespace: Compile
expectation: Pass
*/

record Token {
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    amount: u64,
}

function main() -> bool {
    return true;
}
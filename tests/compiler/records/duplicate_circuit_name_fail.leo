/*
namespace: Compile
expectation: Fail
*/

record Token {
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    amount: u64,
}

circuit Token { // This circuit cannot have the same name as the record defined above it.
    x: u32,
}

function main() -> bool {
    return true;
}
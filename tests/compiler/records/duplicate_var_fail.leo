/*
namespace: Compile
expectation: Fail
*/

record Token {
    // The token owner.
    owner: address,
    // The token owner.
    owner: address, // Cannot define two record variables with the same name.
    // The Aleo balance (in gates).
    balance: u64,
    // The token amount.
    amount: u64,
}

function main() -> bool {
    return true;
}
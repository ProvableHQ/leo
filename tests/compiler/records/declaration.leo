/*
namespace: Compile
expectation: Pass
input_files: inputs/add.in
*/

record Token {
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    amount: u64,
}

function main(a: u8, b:u8) -> u8 {
    return a + b;
}
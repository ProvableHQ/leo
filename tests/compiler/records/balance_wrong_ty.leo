/*
namespace: Compile
expectation: Fail
*/

// This record does define the `balance` variable but with the wrong type.
record Token {
    balance: address,
    owner: address,
}

function main() -> bool {
    return true;
}

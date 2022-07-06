/*
namespace: Compile
expectation: Fail
*/

// This record does define the `owner` variable but with the wrong type.
record Token {
    balance: u64,
    owner: bool,
}

function main() -> bool {
    return true;
}

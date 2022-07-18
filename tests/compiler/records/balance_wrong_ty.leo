/*
namespace: Compile
expectation: Fail
*/

// This record does define the `gates` variable but with the wrong type.
record Token {
    gates: address,
    owner: address,
}

function main() -> bool {
    return true;
}

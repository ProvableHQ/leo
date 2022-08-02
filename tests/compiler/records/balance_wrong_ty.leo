/*
namespace: Compile
expectation: Fail
input_files: inputs/add.in
*/

// This record does define the `gates` variable but with the wrong type.
record Token {
    gates: address,
    owner: address,
}

function main(a: u8, b:u8) -> u8 {
    return a + b;
}
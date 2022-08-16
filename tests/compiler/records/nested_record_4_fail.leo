/*
namespace: Compile
expectation: Fail
*/

record Token3 {
    owner: address,
    gates: u64,
    bar: (Bar, Bar),
}

circuit Bar {
    bar: (Token, Token),
}

record Token {
    owner: address,
    gates: u64,
    amount: u64,
}






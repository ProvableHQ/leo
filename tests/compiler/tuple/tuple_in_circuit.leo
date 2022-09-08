/*
namespace: Compile
expectation: Pass
*/

circuit A {
    mem: (u8, u16)
}

circuit B {
    mems: (A, A)
}

circuit Token {
    owner: address,
    gates: u64,
    amounts: (u64, u64),
}

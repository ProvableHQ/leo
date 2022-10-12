/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    struct A {
        mem: (u8, u16)
    }

    struct B {
        mems: (A, A)
    }

    circuit Token {
        owner: address,
        gates: u64,
        amounts: (u64, u64),
    }
}


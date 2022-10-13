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
}


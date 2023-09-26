/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }
    record Token {
        owner: address,
        amount: u64,
        arg1: u64,
        arg2: u64,
    }

    transition main(a: u8, b:u8) -> u8 {
        return a + b;
    }
}

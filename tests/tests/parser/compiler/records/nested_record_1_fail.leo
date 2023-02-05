/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Foo {
        owner: address,
        gates: u64,
        amount: u64,
    }
    
    record Token {
        owner: address,
        gates: u64,
        foo: Foo,
    }
}

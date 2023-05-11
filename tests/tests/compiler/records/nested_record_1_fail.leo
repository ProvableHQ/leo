/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Foo {
        owner: address,
        amount: u64,
    }
    
    record Token {
        owner: address,
        foo: Foo,
    }
}

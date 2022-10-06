/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Token {
        owner: address,
        gates: u64,
        bar: Bar,
    }
    
    
    struct Bar {
        bar: Foo,
    }
    
    record Foo {
        owner: address,
        gates: u64,
        amount: u64,
    }
}

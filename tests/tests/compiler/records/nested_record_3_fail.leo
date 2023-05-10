/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Token {
        owner: address,
        bar: Bar,
    }
    
    
    struct Bar {
        bar: Foo,
    }
    
    record Foo {
        owner: address,
        amount: u64,
    }
}

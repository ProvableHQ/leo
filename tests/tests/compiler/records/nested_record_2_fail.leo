/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    record Token2 {
        owner: address,
        foo: (Foo, Foo),
    }
    
    record Foo {
        owner: address,
        amount: u64,
    }
}

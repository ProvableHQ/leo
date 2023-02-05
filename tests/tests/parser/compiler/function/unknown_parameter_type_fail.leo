/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition main(a: u8, foo: Foo) -> u8 {
        return a;
    }
    
    transition returns_foo(a: u8) -> Foo {
        return a;
    }}

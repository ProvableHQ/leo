/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    struct Foo {
        a: u8,
        bar: Bar,
    }
    
    transition main(a: u8) -> u8 {
        return a + 1u8;
    }
}

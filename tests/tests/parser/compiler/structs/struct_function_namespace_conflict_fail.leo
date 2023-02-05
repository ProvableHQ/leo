/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    struct Foo {
        x: u8;
    }
    
    function Foo() {}
    
    function main(y: bool) -> bool {
        return y;
    }
}

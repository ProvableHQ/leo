/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    struct Foo {
        x: u8;
    }
    
    function main() {
        let y: u8 = 1;
        let a: Foo = Foo { y };
    }
}

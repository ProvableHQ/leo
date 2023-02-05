/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    struct Foo {
        x: u8;
    }
    
    function main() {
        const y: u8 = 1;
        const a: Foo = Foo { y };
    }
}

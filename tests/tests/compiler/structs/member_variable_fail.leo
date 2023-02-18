/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    struct Foo {
        x: u32;
    }
    
    function main() {
        let a: foo = Foo { x: 1u32 };
        let err = a.y;
    }
}

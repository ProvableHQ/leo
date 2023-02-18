/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    struct Foo {
        x: u32;
    }
    
    function main() {
        // no member y in Foo
        let a: Foo = Foo { y: 0u32 };
    }
}

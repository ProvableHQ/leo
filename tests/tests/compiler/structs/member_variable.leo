/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    struct Foo {
        x: u32,
    }
    
    transition main(y: bool) -> bool {
        let a: Foo = Foo { x: 1u32 };
    
        return (a.x == 1u32) == y;
    }
}

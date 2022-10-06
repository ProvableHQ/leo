/*
namespace: Compile
expectation: Pass
inputs:
 - inline.in: |
    [main]
    x: u32 = 100;

    [registers]
    r0: u32 = 0;
*/

program test.aleo {    
    struct Foo {
        x: u32
    }
    
    transition main(x: u32) -> u32 {
        let a: Foo = Foo { x: x };
        return a.x;
    }
}

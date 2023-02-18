/*
namespace: Compile
expectation: Fail
inputs:
 - inline.in: |
    [main]
    y: bool = true;

    [constants]
    x: u8 = 10;

    [registers]
    r0: bool = false;  
*/

program test.aleo {    
    struct Foo {
        x: u8
    }
    
    function main(let x: u8, y: bool) -> bool {
        let a: Foo = Foo { x };
    
        return (b.x == a.x) == y;
    }
}

/*
namespace: Compile
expectation: Pass
inputs:
 - inline.in: |
    [main]
    x: u8 = 10;

    [registers]
    r0: bool = false;  
*/

circuit Foo {
    x: u8

    function new(x: u8) -> Self {
        return Self { x };
    }
}

function main(x: u8) -> bool {
    const a = Foo { x };
    const b = Foo::new(x);

    return b.x == a.x;
}

/*
namespace: Compile
expectation: Pass
inputs:
 - inline.in: |
    [main]
    y: bool = true;

    [constants]
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

function main(const x: u8, y: bool) -> bool {
    const a = Foo { x };
    let b = Foo::new(x);

    return (b.x == a.x) == y;
}

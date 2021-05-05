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

circuit Foo {
    x: u32
}

function main(x: u32) -> u32 {
    const a = Foo { x: x };
    return a.x;
}

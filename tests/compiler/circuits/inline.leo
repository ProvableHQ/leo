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

@program
function main(x: u32) -> u32 {
    let a: Foo = Foo { x: x };
    return a.x;
}

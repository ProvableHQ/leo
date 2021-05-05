/*
namespace: Compile
expectation: Pass
inputs:
 - member.in: |
    [registers]
    r0: bool = false;  
*/

circuit Foo {
    x: u32,
}

function main() -> bool {
    const a = Foo { x: 1u32 };

    return a.x == 1u32;
}

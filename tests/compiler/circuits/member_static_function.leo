/*
namespace: Compile
expectation: Pass
inputs:
 - static.in: |
    [registers]
    r0: bool = false;  
*/

circuit Foo {
    function echo(x: u32) -> u32 {
        return x;
    }
}

function main() -> bool {
    const a = Foo::echo(1u32);

    return a == 1u32;
}

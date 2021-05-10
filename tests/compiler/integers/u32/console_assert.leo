/*
namespace: Compile
expectation: Pass
inputs:
 - u32.in: |
    [main]
    a: u32 = 2;
    b: u32 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u32, b: u32) -> bool {
    console.assert(a == b);
    return a == b;
}

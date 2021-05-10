/*
namespace: Compile
expectation: Pass
inputs:
 - u32_g.in: |
    [main]
    a: u32 = 2;
    b: u32 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u32, b: u32) -> bool {
    return a == b;
}

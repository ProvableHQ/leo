/*
namespace: Compile
expectation: Pass
inputs:
 - u32_f.in: |
    [main]
    a: u32 = 4;
    b: u32 = 2;
    c: u32 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u32, b: u32, c: u32) -> bool {
    return a - b == c;
}

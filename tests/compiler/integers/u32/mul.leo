/*
namespace: Compile
expectation: Pass
inputs:
 - mul.in: |
    [main]
    a: u32 = 10;
    b: u32 = 20;
    c: u32 = 30;

    [registers]
    r0: bool = true;
*/

function main(a: u32, b: u32, c: u32) -> bool {
    return a * b == c;
}

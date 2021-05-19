/*
namespace: Compile
expectation: Pass
inputs:
 - u64.in: |
    [main]
    a: u64 = 2;
    b: u64 = 4;
    c: u64 = 6;

    [registers]
    r0: bool = true;
*/

function main(a: u64, b: u64, c: u64) -> bool {
    return a + b == c;
}

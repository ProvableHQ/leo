/*
namespace: Compile
expectation: Pass
inputs:
 - u64.in: |
    [main]
    a: u64 = 4;
    b: u64 = 2;
    c: u64 = 2;

    [registers]
    r0: bool = true;
*/

function main(a: u64, b: u64, c: u64) -> bool {
    return a / b == c;
}

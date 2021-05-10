/*
namespace: Compile
expectation: Pass
inputs:
 - mul.in: |
    [main]
    a: u64 = 10;
    b: u64 = 20;
    c: u64 = 30;

    [registers]
    r0: bool = true;
*/

function main(a: u64, b: u64, c: u64) -> bool {
    return a * b == c;
}

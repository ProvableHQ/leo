/*
namespace: Compile
expectation: Pass
inputs:
 - mul.in: |
    [main]
    a: u128 = 10;
    b: u128 = 20;
    c: u128 = 30;

    [registers]
    r0: bool = true;
*/

function main(a: u128, b: u128, c: u128) -> bool {
    return a * b == c;
}

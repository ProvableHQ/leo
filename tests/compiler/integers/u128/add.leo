/*
namespace: Compile
expectation: Pass
inputs:
 - u128.in: |
    [main]
    a: u128 = 2;
    b: u128 = 4;
    c: u128 = 6;

    [registers]
    r0: bool = true;
*/

function main(a: u128, b: u128, c: u128) -> bool {
    return a + b == c;
}
